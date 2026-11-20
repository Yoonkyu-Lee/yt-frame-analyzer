# W1 압축 루프 기록 (segmented_download.rs)

코드 줄 = `#[cfg(test)]` 앞, 빈 줄과 `//` 시작 줄 제외. **rustfmt(기본 폭 100) 적용 후** 수치를 공식으로 쓴다.
(r1 직후 한 줄이 긴 상태에서 272줄이 나왔지만 포맷하면 374줄이라, 줄 수를 유리하게 만드는 서식은 쓰지 않았다.)

| 라운드 | 층 | 코드 줄 | 함수 | 자체 테스트 | bench (throttled / uneven / unthrottled) |
|---|---|---|---|---|---|
| r0 | 원본 | 556 | 19 | 7 (통과) | 1.09s / 6.08s / 0.50s |
| r1 | 모듈+함수 (머리 주석 → 재작성) | 374 (-33%) | 18 | 14 (통과) | 1.10s / 6.10s / 0.53s |
| r2 | 청크 (오류 처리 통일, 재개 상태 한 핸들) | 332 (-11% vs r1, -40% vs r0) | 18 | 14 (통과) | 1.10s / 6.12s / 0.46~0.69s (잡음) |
| 회의론자 | D1~D10 대조 후 테스트 추가 | 332 (코드 변화 없음) | 18 | 15 (통과) | 1.10s / 6.10s / 0.53s |

정지: r2 다음 라운드에서 찾은 후보(Progress 합치기, 취소 레지스트리 등)는 합쳐도 10줄 안팎(<5%)이라 구현하지 않고 멈췄다. 즉 r3는 "실행"이 아니라 눈으로 한 추정이다.

## r1: 모듈 → 함수 (의도 ID 기준)
| 단위 (원본) | 머리 한 줄 | 처리 |
|---|---|---|
| CANCELLERS, register/request/clear | D7 토큰 | 유지, `if let Ok` 방어를 `unwrap`으로 (잠금은 짧고 panic 없음) |
| CANCEL_SENTINEL, `cancelled()` | D7 | **삭제**. 취소를 오류 문자열로 올리던 방식 대신 `download`가 "오류이고 플래그가 켜져 있으면 Ok(None)"으로 변환 |
| ProgressEmitter (CAS, started, last_emit_ms) | D8 | 재작성: `Mutex<Instant>` 하나, 콜백을 잠금 안에서 호출해 received 역행도 막음 (원본은 경쟁 가능) |
| ResumeMeta + JSON + 3개 sidecar (`.download .json .state`) | D4 | **삭제**(비목표: 파일 이름/형식 유지). 임시 파일 꼬리 `[total][구간별 u64]` 한 파일로 통합. serde_json/Deserialize 의존 제거 |
| `valid_resume_offset`, `segment_bounds`(나머지 분배) | D4/D5 | 인라인 / `total*i/n`으로 교체 (빈틈·겹침 없음, 테스트로 고정) |
| `sidecar_path(suffix)` 3종 | D4 | `part_path` 1종 |
| `client()`, `request(method)` | D9 | 합침(`get`만; HEAD/다른 method 호출 없음) |
| `probe_range` 반환 `(bool, Option)`+CONTENT_LENGTH 파싱 | D2 | 삭제 후 `run`에 인라인. 단일 경로는 두 번째 GET의 content_length를 쓰므로 프로브의 Content-Length 불필요 |
| `download_segment` 9인자, 이중 retry 분기 | D5/D6 | `segment`(재시도 루프) + `fetch`(요청 1회)로 분리. 모든 오류를 같은 방식으로 재시도(원본은 Content-Range 불일치·non-206을 즉시 실패로 처리했음) |
| `segmented`의 Arc 8개 복제 | D3 | `Job` 구조체 1개를 `Arc`로 공유 |
| `single` | D2 | 유지 (+ 받은 크기 ≠ 알린 크기이면 Err) |
| `SEGMENTS_PER_CONNECTION`, `PROGRESS_EMIT_INTERVAL`, `RESUME_CHECKPOINT_BYTES` 상수 | - | `SEGMENTS=16`, 250ms 인라인, `CHECKPOINT` |

## r2: 청크
- 오류: `Res<T> = Result<T, Box<dyn Error+Send+Sync>>` 하나로 통일하고 `?`만 쓰다가 `download` 경계에서 `"다운로드 실패: {e}"`로 한 번 감쌈(D10, L10). 약 40개의 `.map_err(format!(..))`가 사라짐. 대가: 문장 뒷부분은 라이브러리 원문(영어)이고 "어느 단계"의 한국어 맥락이 줄었다 (Weak에 적음).
- 재개 상태 파일을 데이터와 같은 핸들에 써서 `Mutex<File>` 제거. 같은 핸들이라 데이터 쓰기 → 상태 쓰기 순서가 보장됨(원본은 다른 핸들이라 순서 보장 없음).
- `put(f, at, bytes)` 헬퍼 하나로 seek+write 반복 제거 (청크마다 seek하므로 성능을 bench로 확인: 변화 없음).
- 완료 시 꼬리 truncate는 `open_part`가 돌려준 핸들을 재사용.

## 일부러 남긴 것
- 구간 16개/연결 8개(원본 구조): P1 uneven(6.1s)이 이 구조에 의존. 알고리즘은 바꾸지 않음.
- 체크포인트 1MiB (D4 "연결당 최대 1MiB 재요청").
- `n > len - done` 초과 본문 검사: **방어(c)**. 이 검사를 빼도 마지막에 `done != len`으로 Err가 나서 테스트는 통과했다(변이 테스트로 확인). 하지만 초과분이 이웃 구간 데이터를 덮어쓴 채 이웃의 완료 체크포인트가 남을 수 있어(D5 위험) 남겼다. 결정적 테스트로는 고정하지 못했다.
- 취소 시 `f.flush()` (재개 상태가 디스크에 닿도록).
- `.referer(false)`: reqwest가 리다이렉트마다 Referer를 덮어써 D9("referer를 보낸다")를 어기던 것을 테스트(리다이렉트 서버)로 발견해 고침. **원본에도 있던 결함**이라 압축이 만든 퇴행이 아니다.

## 의도 변화/주의 (원본과 다른 동작)
- 구간 응답이 non-206/Content-Range 불일치여도 재시도한다(최대 약 3초). 원본은 즉시 Err. D5/D6에는 부합.
- 프로브가 416이면 단일 경로로 넘어가 빈 파일(D1 0바이트)을 받는다. 원본은 416에서 Err.
- 원본 sidecar 3개 → 1개. 이름·형식은 비목표.

## 회의론자 (D1~D10 대조: 보장하는 코드 / 고정하는 테스트)
| 의도 | 코드 | 테스트 |
|---|---|---|
| D1 정확한 바이트, 0바이트, 깨끗한 폴더, 완성 후에만 목적지 | `segmented`/`single` 끝의 truncate+rename, 416 → single | `parallel_download_is_exact_clean...`, `tiny_and_empty_files_are_exact`(0/1/5/17바이트) |
| D2 프로브, 206→분할, 그 외 단일, HTTP 오류 Err | `run` | 첫 요청 `range: bytes=0-0`(parallel 테스트), `no_range_or_unknown_total...`(200, `bytes 0-0/*`), `http_error_is_err_without_files` |
| D3 연결 ≤ 8 | `CONNECTIONS.min(n)`개 워커 | 서버가 동시 연결 최대치를 재서 2..=8 확인 (CONNECTIONS를 16으로 바꾸면 실패: 변이 확인) |
| D4 재개, 다른 크기 상태 폐기 | `open_part` | `cancel_midway_then_resume...`(재요청량 ≤ 75%), `foreign_or_damaged_resume_state_is_discarded` (다른 크기/오프셋 손상/길이 어긋남) |
| D5 잘못된 범위/긴 본문/짧은 본문/손상 상태 | `fetch`의 Content-Range·초과·크기 검사, `open_part` 검증 | `wrong_range_long_body_and_short_body_never_complete`(+실패 후 이어받아도 정확), 변이: Content-Range 검사 제거 시 실패, 오프셋 검증 제거 시 실패. 초과 본문 검사는 위 "남긴 것" 참조 |
| D6 재시도, 수 초 내 Err | `segment` 루프(500ms·k 백오프, 4회) | `transient_failures_are_retried`, `persistent_failure_ends_in_seconds`(<10s) |
| D7 취소 | `fetch` 청크마다 확인, `download`의 변환 | `preset_cancel...`(구간 요청 없음, part 유지), 중간 취소(재개 테스트), `cancel_during_single_stream...`, 토큰 레지스트리 테스트 |
| D8 진행 | `Progress` | `progress_is_monotonic_rate_limited_and_ends_complete`(단조, total, mode, 초당 ≤4+3, 마지막 received==total), 단일 경로 마지막 이벤트 |
| D9 헤더/리다이렉트 | `get`, `run`의 client | `requests_carry_required_headers...`(모든 요청의 identity·cookie·referer·Chrome UA, 302 추적, 빈 문자열은 생략). 연결 타임아웃 20s와 리다이렉트 10회 상한은 **테스트 없음** (코드로만 보장) |
| D10 한국어 오류 | `download` 경계 | 실패 테스트들이 비ASCII(한글) 포함 확인 |

변이(코드를 일부러 깨 보고 테스트가 잡는지) 확인: CONNECTIONS=16, Content-Range 검사 제거, 오프셋 검증 제거, `.referer(true)`, 체크포인트 미기록 → 모두 실패로 잡힘. 초과 본문 검사 제거만 못 잡음.

## 불확실한 점
- 테스트 서버는 직접 만든 스레드 서버라 실제 호스트의 거동(chunked 인코딩, 연결 재사용)은 다루지 않는다.
- 재개 정확도는 "같은 URL은 같은 바이트"라는 가정(INTENT)에 의존.
- 크래시(하드 종료)로 체크포인트가 데이터보다 앞서는 경우는 재현하지 못했다.
- unthrottled bench는 실행마다 0.46~0.69s로 흔들린다(원본 0.50s).
