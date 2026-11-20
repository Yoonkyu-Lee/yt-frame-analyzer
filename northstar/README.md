# northstar

바이브 코딩으로 커진 프로젝트를 hand-remade의 기조로 되돌리는 원칙(의도 밀도)과 워크플로(**리캐스트**)입니다. 리캐스트는 의도를 거푸집으로 삼아 코드를 다시 붓는 일입니다. 리팩토링처럼 동작을 보존하지 않고, 의도에 맞게 동작을 다시 정합니다. Hayyoth 손코딩 분석([sources/hayyoth](../sources/hayyoth/))과 Codex 대조 실험([experiments/vibe-coding](../experiments/vibe-coding/))에서 확인한 사실을 근거로 삼고, 외부 출처([sources/](../sources/))에서 들여와 아직 확인하지 않은 것은 *(가설)*로 표시합니다.

**한 문장 요약**: 모든 코드는 코드 밖에 적힌 의도나 측정된 필요까지 거슬러 올라갈 수 있어야 하고, 그 조건을 만족하는 가장 작은 코드가 정답이다.

| 파일 | 대상 | 내용 |
|---|---|---|
| [NORTHSTAR.md](NORTHSTAR.md) | 사람 | 의도 밀도 개념, 의도 그래프, 개념 12개, 법칙 16개(L13~L16은 가설), 우연 코드의 냄새, 적용하지 말아야 할 곳 |
| [WORKFLOW.md](WORKFLOW.md) | 사람 | 리캐스트 워크플로: 역할, 0~8단계(오라클, 슬롭 진단, 혼합 리캐스트, 회의론자 라운드, 실사용 확인, 상시 회의 루프), 게이트, 지표 |
| [AGENTS.md](AGENTS.md) | AI | 대상 프로젝트 루트에 복사해서 쓰는 규칙(영어. Codex는 `AGENTS.md`, Claude는 `CLAUDE.md`로) |
| [templates/](templates/) | 둘 다 | `INTENT.md`, `TRACE.md`, `LEDGER.md`, `CHANGE.md`, `ORACLE_CHANGES.md`, `ROUND.md` |
| [gate.py](gate.py) | CI / AI | git diff에서 추적, 의도, 성장 예산, 의존성, 수정 원인, 돌연변이 검사, 래칫, 대안, 변명 주석을 검사. 표준 라이브러리와 git만 사용 |
| [pilot-05/](pilot-05/README.md) | 사람 | WORKFLOW 4단계의 의미 압축 루프를 Codex의 파일 유틸 812줄에 적용한 실험. 라운드별 기록, 대조군, 회의론자 리뷰 |
| [pilot-vnovel/](pilot-vnovel/README.md) | 사람 | 실제 바이브 코딩 앱의 다운로드 모듈에 워크플로 4종을 대결시키고, 회의론자 라운드를 거쳐 main에 병합한 실험 |
| [LESSONS.md](LESSONS.md) | 사람 | 두 파일럿에서 배운 것, 외부 출처에서 들여온 가설, 그것이 모델의 어디에 반영됐는지 |
| [skill/](skill/README.md) | AI | 리캐스트를 Claude Code 스킬로 묶은 것 (설치 전, 아직 사용하지 않음) |

## 시작하기

1. 대상 프로젝트 루트에 `AGENTS.md`(필요하면 `CLAUDE.md`), `templates/`의 네 파일, `gate.py`를 복사합니다.
2. [WORKFLOW.md](WORKFLOW.md) 0단계(기준선)와 1단계(의도 복원)를 먼저 끝냅니다. **`INTENT.md`의 승인은 사람이 합니다.**
3. 이후 모든 변경은 변경 메모와 함께 만들고, `python gate.py --base <브랜치>`를 통과시킵니다.

## 게이트 시연

hand-completed → hand-remade 변경을 임시 git 저장소에 그대로 올려 돌려 봤습니다.

| 상황 | 결과 |
|---|---|
| 리메이크 변경 + 바이브식 helper(`util.c`)를 추적 없이 추가, 변경 메모 없음 | **FAIL**: trace(`util.c`), intent(메모 없음), budget(순 +91줄, 예산 30) |
| helper 제거, 변경 메모(`Intent:`, `Budget:` 근거, `Evidence:`) 추가 | **PASS** (+148 −58, 삭제 비율 0.39) |
| Makefile에 `-lm` 추가, LEDGER 기록 없음, 원인 없는 수정 메모 | **FAIL** deps, **WARN** fix |
| LEDGER 기록 + `Root cause:`, `Deletion considered:` 추가 | **PASS** |
| *(2026-11-17, 임시 저장소)* 새 파일에 `// workaround: release later, in case it comes back`, 수정 메모에 `Ratchet:`·`Alternatives:` 없음 | **WARN** ratchet, alternatives, excuse |
| 변명 주석을 지우고 메모에 `Ratchet:`(막는 검사)와 `Alternatives:` 추가 | **PASS** |

게이트가 막는 것은 코드 자체가 아니라 **이유 없는 코드**입니다. 줄이 늘어도 무엇을 사는지 적혀 있으면 통과합니다.

## 한계

- 게이트는 기계적으로 확인할 수 있는 법칙(L1, L2, L4, L6, L13, L14)만 봅니다. L13·L14 검사는 메모에 필드가 있는지와 주석의 낱말만 보므로, 그 내용이 참인지는 회의론자가 봅니다. 추상화의 적절성(L3), 경계 방어(L12), 한 화면 단위(L9)는 회의론자 리뷰의 몫입니다.
- `INTENT.md`가 틀리면 모든 것이 틀립니다. 의도 복원(1단계)은 AI가 초안을 쓰더라도 사람이 승인해야 합니다.
- 상시 회의 루프(8단계)와 L13~L16은 외부 발표 하나에서 들여온 가설이고, 아직 어떤 프로젝트에서도 돌려 보지 않았습니다([LESSONS.md](LESSONS.md) 마지막 절).
- 이 워크플로는 아직 큰 실제 프로젝트 전체에 적용해 보지 않았습니다. 근거는 hand-remade, 게이트 시연, pilot-05(합성 812줄), pilot-vnovel(실제 앱의 모듈 하나, main 병합)입니다.
