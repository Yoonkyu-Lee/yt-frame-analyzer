# recast 스킬 (설치하지 않은 상태)

[WORKFLOW.md](../WORKFLOW.md)의 리캐스트 절차를 Claude Code 스킬로 묶은 것입니다. **아직 어디에도 설치하지 않았습니다.** 이 폴더는 Claude Code가 스킬을 찾는 위치(`.claude/skills/`, `~/.claude/skills/`)가 아니라서 자동으로 걸리지 않습니다.

## 구성

| 경로 | 내용 |
|---|---|
| [recast/SKILL.md](recast/SKILL.md) | 진입점: 기본 규칙, "지금 어디인가"(파일로 진행 상태를 판단해 이어 가기), 0~7단계(7 = 상시 회의 루프, 가설), 사람에게 묻고 멈추는 지점(⛔) |
| `recast/references/` | 단계별 세부 절차: intent, oracle, triage, recast, skeptic, integrate, doubt, lessons. 그 단계에 들어설 때만 읽음 |
| `recast/prompts/` | 회의론자 프롬프트(회귀 리뷰, 근본 회의), 실험군 지시문(공통, 압축, 재도출, 혼합, 대조군), 블라인드 독자 |
| `recast/scripts/gate.py` | 변경 게이트 (`northstar/gate.py` 사본) |
| `recast/scripts/mutate.py` | 돌연변이 검사: 고친 자리를 한 번 되돌려 테스트를 돌리고 항상 원상복구. 자체 검사 통과 |
| `recast/templates/` | INTENT, TRACE, LEDGER, CHANGE, ORACLE_CHANGES, ROUND (`northstar/templates` 사본) |

스킬 안의 지시문은 영어입니다. AI가 읽는 규칙이라 `AGENTS.md`와 맞췄습니다. `gate.py`와 템플릿은 사본이므로, `northstar/`에서 고치면 여기에도 복사해야 합니다.

## 쓰려면 (나중에)

```sh
# 모든 프로젝트에서
cp -r northstar/skill/recast ~/.claude/skills/recast
# 한 프로젝트에서만
cp -r northstar/skill/recast <project>/.claude/skills/recast
```

회의론자는 Codex CLI(`codex exec -s read-only`, 표준 입력은 닫음)를 기본으로 씁니다. Codex가 없으면 대화를 보지 않은 별도 Claude 에이전트로 대신합니다.

## 아직 검증하지 않은 것

이 스킬로 실제 리캐스트를 해 본 적은 없습니다. 내용은 두 파일럿을 제가 손으로 진행한 기록에서 뽑았습니다. 다음 확인 방법으로는 같은 대상(예: pilot-05)을 스킬로 다시 돌려 손으로 한 결과와 비교하는 것이 좋습니다.
