# yt-frame-analyzer

대본이나 오디오 없이 **프레임만 판독해서** YouTube 영상을 해체 분석하고, 거기서 얻은 것을 AI와 함께 코드를 만드는 방식(northstar)으로 모으는 리포입니다.

흐름은 세 층입니다. **출처**(영상을 해체한 원자료) → **실험**(출처의 주장을 우리 손으로 확인) → **northstar**(검증된 것만 모은 원칙과 워크플로).

## 구조

```
sources/                        출처: 영상 해체 분석. 채널(발행 주체)마다 폴더 하나
  README.md                     모든 출처의 목록, 출처마다 배운 것, 출처끼리 겹치고 부딪치는 곳
  hayyoth/                      Hayyoth 손코딩 채널 26편 전체 (2026-11-17 기준)
    README.md, PHILOSOPHY.md    영상 목록과 요약, 이 채널의 철학(근거 영상 수와 함께 누적)
    NN_<slug>_<videoId>/        영상별 결과물 (NN = 채널 업로드 순서)
  ai-engineer/                  AI Engineer 컨퍼런스 발표
  agentos/                      AgentOS (한국어 해설 채널)
    <slug>_<videoId>/           채널 전체가 아니라 골라 본 영상은 번호 없이 slug와 ID만
experiments/                    실험: 출처의 주장을 같은 조건에서 재현하거나 비교
  vibe-coding/                  Hayyoth 과제를 맥락 없이 Codex에게 맡기고 사람 코드와 비교
northstar/                      종합: 바이브 코딩 프로젝트를 hand-remade 기조로 되돌리는 원칙(의도 밀도)과 워크플로(리캐스트)
  NORTHSTAR.md, WORKFLOW.md     개념과 법칙, 리캐스트 절차
  AGENTS.md, templates/, gate.py  대상 프로젝트에 복사해서 쓰는 AI 규칙, 문서 템플릿, 변경 게이트
  pilot-05/, pilot-vnovel/      리캐스트 파일럿 두 개 (합성 812줄, 실제 앱 모듈 → main 병합)
  LESSONS.md                    두 파일럿에서 배운 것과 모델에 반영한 것
  skill/recast/                 리캐스트 Claude Code 스킬 (설치 전)
tools/
  prepare.py                    영상 → 키프레임 → 구간 분할 (준비 단계 전체)
  contact_sheet.py              프레임을 고르게 뽑은 4×N 모음 이미지 (레이아웃과 웹캠 위치 확인용)
  extract_state.py              판독 원문의 파일 상태 블록을 추출해 vim 체크섬(줄 수, 바이트)과 대조
  reindent.py                   중괄호 구조에서 들여쓰기를 다시 계산 (판독이 들여쓰기를 한 칸씩 틀렸을 때만 사용,
                                --selftest로 검증된 파일을 원본 그대로 재현하는지 먼저 확인할 것)
  prompts/transcribe_code.md    구간 판독 프롬프트: 코딩 영상 (파일 상태, 체크섬)
  prompts/transcribe_talk.md    구간 판독 프롬프트: 발표, 강의, 화면 글 영상 (슬라이드, 자막, 메모)
work/<videoId>/                 캐시: 영상, 프레임, 키프레임 (git 제외, 지워도 재생성 가능)
work/repos/                     캐시: 분석에 참고한 외부 저장소 클론 (git 제외)
```

새 채널의 영상은 `sources/<채널>/` 폴더를 만들고, [sources/README.md](sources/README.md)의 목록에 한 줄을 더합니다. 한 채널의 주장은 그 채널 폴더 안에만 적고, 여러 출처를 근거로 하는 원칙만 northstar로 올립니다.

## 새 영상 분석 절차

1. **준비**: 메타데이터, 1440p 다운로드, 1fps 프레임, 변화 감지, 키프레임, 8구간 분할을 한 번에 실행합니다.
   ```sh
   python tools/prepare.py <url>                          # 1차 실행
   python tools/contact_sheet.py work/<id> 16             # 레이아웃 확인 → work/<id>/contact.jpg
   python tools/prepare.py <url> --mask 0.69,0.26,0.985,0.86 --segments N  # 웹캠을 가리고, 구간당 키프레임 약 40장이 되도록 N을 정함
   ```
   - 1차 실행 뒤 `work/<id>/frames/`에서 몇 장을 열어 레이아웃을 확인하세요.
   - 웹캠처럼 계속 움직이는 영역은 `--mask`(정규화 좌표)로 가립니다.
   - 다운로드와 프레임 추출은 캐시되므로, 마스크와 임계값(`--th`)만 바꿔 다시 돌리는 비용은 작습니다.
   - 18번부터는 웹캠이 화면 오른쪽 아래 끝까지 내려가 `--mask 0.69,0.26,1.0,1.0`을 씁니다.
   - vim 저장 메시지(체크섬)가 있는 영상은 기본 임계값(0.004)을 유지합니다. 줄 번호도 저장 메시지도 없는 GUI 편집기의 긴 영상(18, 21, 23번)은 `--th 0.02~0.05`로 올려 키프레임 수를 줄입니다. 중간 타자 과정 일부를 건너뛰는 대신 비용이 2~3배 줄어듭니다.
   - 발표 영상은 연사 카메라 영역을 가립니다. 슬라이드가 바뀔 때만 키프레임이 생깁니다.
2. **판독**: 영상 종류에 맞는 프롬프트([코딩](tools/prompts/transcribe_code.md), [발표·화면 글](tools/prompts/transcribe_talk.md))를 지시문으로, 영상 정보와 구간 번호를 실행 메시지로 주고 구간마다 Sonnet 에이전트를 1개씩 병렬로 실행합니다. 결과는 `sources/<채널>/<폴더>/transcripts/segN.md`에 저장됩니다.
3. **통합**: 구간별 파일 상태를 합쳐 `src/`를 복원합니다.
   - 에이전트끼리 결과가 어긋나는 곳, 마지막 화면, 탭 제목 같은 핵심 프레임은 원본 해상도로 직접 확인합니다.
4. **검증**: 화면에 찍힌 체크섬과 실제 실행으로 복원본을 검증합니다.
   - 편집기 저장 메시지(vim의 `"file" NL, NB written` 등)가 있으면 체크섬으로 써서 `history/`의 버전별 크기를 대조합니다(`verify/`).
   - 컴파일하고 실행해서 영상 속 경고, 버그, 출력이 재현되는지 확인합니다. C는 WSL의 gcc로 돌립니다.
   - 발표 영상은 코드 대신 **주장**을 검증합니다. 화면에 근거(숫자, 스크린샷, 공개 저장소)가 있는 주장과 말뿐인 주장을 나누고, 공개 저장소가 있으면 주장과 실제 내용을 대조합니다.
5. **정리**:
   - 영상 폴더의 `README.md`에 분석을 씁니다: 신뢰도, 타임라인, 코드나 주장 해설, 한계.
   - 채널 폴더의 `README.md`(와 채널 철학 문서가 있으면 그것)를 갱신합니다. Hayyoth는 [PHILOSOPHY.md](sources/hayyoth/PHILOSOPHY.md)에 관찰과 근거 영상 수를, [README.md](sources/hayyoth/README.md)에 목록·요약·공통 패턴을 갱신합니다.
   - [sources/README.md](sources/README.md)에 출처 한 줄과, 다른 출처와 겹치거나 부딪치는 점을 적습니다.

## 필요한 것

- Python 3 + `pip install yt-dlp imageio-ffmpeg opencv-python-headless numpy pillow`. ffmpeg는 imageio-ffmpeg에 포함된 바이너리를 씁니다.
- 복원한 코드를 실행하려면 해당 언어 툴체인이 필요합니다. C/POSIX 코드는 WSL Ubuntu를 씁니다.
