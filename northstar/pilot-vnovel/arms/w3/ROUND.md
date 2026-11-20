# W3 (hybrid) ROUND.md

## 1. 코어 분석 (원본)
- 분할: 파일을 고정 16개 구간(8연결 x 2)으로 균등 분할, 8개 worker가 `AtomicUsize`로 다음 구간 번호를 가져간다. 구간은 한 번 배정되면 끝까지 그 연결이 받는다.
- 재개 상태: 3개 sidecar(`.download` 본체, `.download.json` 메타(total, segment_count, version), `.download.state` 구간별 u64 LE 완료량). 구간 오프셋이 구간 길이를 넘으면 0으로 되돌리는 검증.
- 완료 판정: 모든 worker가 Ok이면 곧바로 rename (구간 길이 일치 검사는 구간별 `completed == segment_len`).
- 측정(원본): 테스트 7 통과, bench `1.09s | 6.09s | 0.49s`.
- 정당화 여부: `uneven`은 6.09s인데, 구간이 5 MB이고 느린 연결(1 MB/s)에 걸린 구간은 5초가 걸린다. 빠른 연결은 일이 끝나면 놀므로 꼬리 시간 = 느린 구간 하나. 고정 분할은 "연결 속도는 열어 봐야 안다"는 사실을 설계에 반영하지 못한다. 의도(P1)는 "회귀 없음"이라 6.09s도 통과지만, 측정이 코어 약점을 가리킨다. 또 사이드카 3개/포맷 2종/오프셋 검증 함수는 "남은 구간 목록" 하나로 환원된다.
- 시도하지 않은 대안(근거): 작은 고정 조각(1 MiB)을 큐로 배정하는 방식은, 서버가 응답마다 연결을 닫는 환경에서 조각마다 새 연결(1/3이 느림)이라 계산상 약 4s로 개선 폭이 작다. 그래서 "긴 구간 + 끝에서 나눠 가져오기(steal)"를 골랐다.

## 2. 코어 결정 (의도에서 재도출)
- 데이터 표현: 남은 구간 목록 `[start,end)` = 미배정 `todo` + 연결별 진행 중 `slots`(start가 곧 재개 오프셋). 스케줄링 = `claim`: todo 우선, 없으면 가장 많이 남은 연결의 뒤 절반을 가져온다(그 연결은 줄어든 end에서 멈춤). 8개 연결 상한(D3)은 slot 8개.
- 재개 상태 = 같은 목록을 적은 4096바이트 고정 텍스트 파일 1개(첫 줄 total, 구간들, `end` 표식). 완료 판정 = 남은 목록이 비었는가(D1/D5 안전망), 상태는 항상 실제보다 덜 진행된 쪽이라 낡아도 안전.
- 상태 갱신: 처음엔 임시파일+rename으로 구현했으나 512 MB 무스로틀 bench가 0.97s로 회귀(Windows에서 rename ~1ms x 512회, mutex 안). 제자리 덮어쓰기 + `end` 표식으로 교체 후 0.4s.
- MIN_STEAL 측정: 128 KiB 2.32s / 256 KiB 2.5s / 1 MiB 3.44s. 128 KiB 채택.
- 측정 결과: `uneven` 6.09s -> 2.32s.

## 3. 라운드 표
| 라운드 | 코드줄 | 함수 | 자작 테스트 | bench (throttled / uneven / unthrottled) |
|---|---|---|---|---|
| 0 원본 | 556 | 19 | 7 | 1.09 / 6.09 / 0.49 |
| 1 코어 재도출 + 주변부 압축을 한 번에 | 371 | 19 | 14 | 1.07 / 2.31 / 0.42 (체크포인트 교체 후) |
| 2 자기비판 보강 (테스트 추가, 코드 +0) | 371 | 19 | 16 | 1.08 / 2.32 / 0.43 |
(주의: 코어가 바뀌어 1라운드에서 주변부도 같이 다시 썼기 때문에 "주변부만의 감소량"은 따로 측정하지 못했다. 이후 한 번 더 줄일 후보를 훑었으나 5% 미만이라 중단.)

## 4. 주변부: 삭제/재작성/유지
- 삭제: 3 sidecar 중 2개와 `ResumeMeta`+serde 직렬화(`serde_json` 의존 불필요, 텍스트 목록으로 충분; D4), `valid_resume_offset`/`segment_bounds`/`prepare_resume`(코어 교체로 소멸; D5는 `load_state` 검증+`end` 표식으로), `CANCEL_SENTINEL`(취소 여부는 플래그로 판단; D7), `ProgressEmitter` 구조체와 `AtomicU64` 이중 카운터(mutex 안의 `done`/`last_emit`로 통합; D8), `parse_content_range`(probe에 인라인), `probe_range`의 불필요한 Content-Length 분기(단일 스트림 응답이 자체 길이를 가짐), 재시도 로그 `eprintln!`, `SEGMENTS_PER_CONNECTION`.
- 재작성: 취소 레지스트리(중독 처리 헬퍼 1개, 3함수 한 줄씩), 요청 생성 `get`(D9), 단일 경로(임시파일 + 실패 시 삭제 + 길이 불일치 검사), 에러 문자열.
- 유지: 상수들(이름과 의도 ID 주석), 공개 타입 파생 trait, `Content-Range` 시작 검증과 초과 본문 검출(D5).
- 의도하지 않은 동작 변화(원본 대비): (a) 단일 스트림이 실패/취소 시 임시파일을 지운다(재개 불가라 잔여물만 남기 때문, D1). (b) 구간 요청이 206이 아니면(5xx 등) 즉시 실패 대신 재시도(D6). (c) 응답 후 `referer(false)`로 리디렉트 뒤에도 호출자 Referer 유지(원본은 reqwest가 리디렉트 시 Referer를 덮어썼음: 새로 찾은 D9 결함, 테스트로 확인). (d) 상태/본체 파일 이름은 같으나 `.download.json` 폐지(비목표에서 허용).

## 5. 자기비판: D1-D10 보증 코드와 자작 테스트
| 의도 | 보증 코드 | 테스트 |
|---|---|---|
| D1 | rename 완료 시에만 목적 파일 생성, 성공 시 상태 삭제, 남은 목록 비어야 rename | exact_bytes_and_no_leftovers_for_any_size (0/1/7/100/1M 바이트), 실패 시 파일 없음은 http_error/persistent 테스트 |
| D2 | `probe` (206+총량 -> 분할, 그 외 성공 -> 단일, 오류 -> Err) | range_ignored_or_unknown_total_uses_single_stream, http_error_is_korean_err, probe_first_and_every_request |
| D3 | slot 8개 | uses_several_connections_but_at_most_eight (서버 측 peak 2..=8) |
| D4 | 상태 파일 + `load_state` | cancel_midway_then_resume_fetches_only_missing, saved_state_limits_requests_to_remaining_spans, foreign_or_damaged_state_is_discarded |
| D5 | Content-Range 시작 검증, 초과 본문 Err, 짧은 본문 재시도, `load_state` 검증, `end` 표식 | wrong_range_or_oversize_never_yields_wrong_file, transient_drops_retry..., foreign_or_damaged_state |
| D6 | `fetch` 4회 + 0.5/1/1.5s 대기 | transient_drops_retry_and_persistent_failure_errs_quickly |
| D7 | 청크마다 취소 검사, `Ok(None)` 변환, 레지스트리 | preset_cancel..., cancel_midway..., cancel_during_single_stream..., cancel_tokens_register_request_clear |
| D8 | mutex 안 `done`(단조) + 250ms 제한 + 마지막 `emit(total)` | progress_is_monotone_rate_limited_and_ends_at_total (+ 단일 모드는 range_ignored 테스트의 마지막 이벤트) |
| D9 | `get` + 클라이언트 설정 | probe_first_and_every_request_carries_headers (리디렉트 포함), empty_cookie_and_referer_are_not_sent. 20 s connect timeout, 리디렉트 10회는 코드로만 보증(테스트 없음) |
| D10 | 모든 오류 문자열 한국어 | http_error 테스트가 한글 포함 확인 (나머지 메시지는 검사하지 않음) |
추가: claim_takes_todo_then_steals_half_then_gives_up (스케줄링 단위 테스트).

찾아낸 약점과 한계(Weak에 기재):
- steal은 끊은 연결의 이미 서버가 보낸 바이트를 버린다. 무스로틀 로컬 서버에서 재개 테스트가 남은 5.9 MB 대비 최대 ~2.2 MB를 더 받았다(8MiB 허용 안에 있으나 원본보다 낭비 큼). 테스트는 `left + 8 MiB`만 단정한다.
- 상태 파일은 제자리 덮어쓰기라 OS 크래시 시 섹터 단위 찢김 가능성이 이론상 있다(`end` 표식과 파싱 검증으로 대부분 걸러내나 보장은 아님). 데이터 바이트가 디스크에 먼저 닿는 순서도 fsync로 보장하지 않는다.
- 읽기 타임아웃 없음(원본도 없음). 같은 크기의 다른 파일 상태는 구분 못 함(의도의 Assumes).
- 청크 쓰기를 mutex 안에서 블로킹으로 수행: 512 MB bench 0.4s로 문제 없었으나 매우 빠른 디스크/연결에서는 병목 가능.
- 테스트 서버는 응답을 한 번에 쓰므로 Content-Range 오프셋 오류 등 일부 시나리오는 실제 서버와 다를 수 있다. 초과 본문은 마지막 청크와 같은 청크일 때만 Err이고 분리되면 올바른 바이트로 성공한다(D5 허용 범위).
