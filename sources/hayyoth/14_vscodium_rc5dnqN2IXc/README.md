# 해체 분석: "vscode - but better"

- 영상: https://youtu.be/rc5dnqN2IXc (5:39), 태그 `[codium]`
- 결과물: 설정 과정 정리 (이 README), 전사 [transcripts/](transcripts/) (5구간, `settings.json`과 읽은 xv6 `cat.c` 원문 포함)
- 방법: 프레임 338장 → 웹캠을 가린 키프레임 249장 → 5구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다.

## 1. 한 일

화면에 이유를 적은 메모는 없습니다. 무엇을 끄는지가 곧 메시지입니다.

| 시각 | 한 일 |
|---|---|
| 0:00~1:05 | "vscode"를 검색했다가 **VSCodium**으로 갑니다. GitHub 소개문 "binary releases of VS Code without MS branding/telemetry/licensing"을 강조하고, `brew install vscodium`으로 설치합니다 |
| 1:05~2:02 | 화면을 비웁니다. 활동 표시줄, 상태 표시줄, 탭 표시줄, 제목 표시줄 항목(탐색, 명령 센터, 레이아웃, 편집기 동작), 시작 화면, 탐색기의 보조 보기(개요, 타임라인)와 툴바 아이콘을 끕니다. 명령 팔레트로 xv6-riscv 폴더를 엽니다 |
| 2:02~3:05 | "제한 모드" 배너를 없애려고 Stack Overflow 답을 찾아 `"security.workspace.trust.enabled": false`를 넣습니다. 밝은 테마로 바꾸고 파일 아이콘 테마를 "None"으로 둡니다 |
| 3:05~4:06 | 미니맵, 이동 경로, 고정 스크롤, 줄 번호, 글리프 여백, 패널 탭(문제, 출력, 디버그 콘솔)을 끕니다. 글꼴 14, 자동 저장 `afterDelay` |
| 4:06~5:37 | 터미널 패널의 버튼도 정리하고, `codium` 명령을 PATH에 넣어 `lab` 폴더에서 `codium .`으로 엽니다. C로 Hello World를 써서 `cc main.c && ./a.out`, 주석 **`// Look good now, beauty, clean`** |

## 2. 해석

- **"but better"는 기능을 더한 것이 아니라 뺀 것입니다.** 결과는 글자와 터미널만 남은 창으로, 그가 평소 쓰는 vim과 터미널 화면과 거의 같아집니다. 도구가 코드보다 앞에 나서지 않게 하려는 것입니다.
- **원격 측정이 없는 공개 빌드를 고릅니다.** VS Code 대신 VSCodium을 고른 이유는 소개문 한 줄(마이크로소프트 원격 측정과 라이선스 제외)입니다. 12번에서 말한 자유 소프트웨어 태도(Stallman)와 15번의 "No Apple"과 같은 방향입니다.
- **문제를 1차 자료로 풉니다.** 막히면(제한 모드) 검색해서 설정 한 줄로 해결합니다. 확장 기능은 하나도 설치하지 않습니다.
