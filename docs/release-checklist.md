# 공식 CLI 출시 체크리스트

이 문서는 `nodit-cli`를 회사 공식 CLI로 공개할 때 필요한 기준을 정리합니다.

핵심 원칙은 단순합니다.

- 모든 API를 100% 커버하는 것이 목표는 아님
- 대신 회사가 이름을 걸고 `지원할 수 있는 범위`가 명확해야 함
- 그 범위 안에서는 설치, 인증, 실행, 오류 처리, 문서가 안정적으로 동작해야 함

즉, 정식 출시(`GA`) 기준은 `기능 수`보다 `지원 가능한 품질`입니다.

## 출시 단계

### Developer Preview

탐색적 공개 단계입니다.

- 기능 방향 검증 가능
- `raw` 의존이 많아도 됨
- 문서와 동작이 일부 어긋날 수 있음
- 지원 범위가 좁거나 자주 바뀌어도 됨

이 단계에서는 `공식 지원 제품`보다 `실험적 도구`에 가깝습니다.

### Beta

정식 출시 직전 단계입니다.

- 지원 범위가 문서로 고정돼 있어야 함
- 대표 시나리오는 실제 Nodit 환경에서 검증돼야 함
- 설치, 인증, 에러 메시지가 운영 가능한 수준이어야 함
- breaking change는 제한적으로만 허용

이 단계부터는 외부 사용자를 대상으로 안내해도 되지만, 아직 제품 계약 수준의 안정성을 약속하긴 어렵습니다.

### GA

정식 출시 단계입니다.

- 회사가 공식 문서와 지원 채널에서 안내 가능
- 릴리스마다 회귀 검증이 자동화돼 있어야 함
- 문서, 설치, CLI UX, 오류 처리, 버전 정책이 정리돼 있어야 함
- 지원 범위 밖 기능은 명확히 구분돼 있어야 함

## Must-have

정식 출시 전 반드시 충족해야 하는 항목입니다.

### 1. 지원 범위 문서화

- 체인별, 제품별, 명령별 지원 상태가 명확해야 함
- 각 항목을 `supported`, `experimental`, `raw-only`, `not supported`로 구분
- typed command가 없는 경우 `raw`로만 가능한지 여부를 문서에 명시

정식 CLI는 `무엇을 지원하는지`보다 `무엇을 지원하지 않는지`가 더 명확해야 합니다.

### 2. 설치와 인증의 안정성

- Linux, macOS, Windows 설치 경로가 모두 문서와 실제 동작이 일치해야 함
- API key 설정 방법이 `--help`, README, 예제 설정 파일에 모두 반영돼야 함
- 인증이 없으면 즉시 실패해야 하고, 실패 메시지는 다음 행동을 바로 안내해야 함
- 설치 직후 첫 실행 경험이 매끄러워야 함

### 3. CLI 인터페이스 안정화

- 명령명과 플래그명이 릴리스마다 자주 바뀌면 안 됨
- 출력 envelope와 exit code가 일관돼야 함
- 오류 메시지가 재현과 triage에 도움이 돼야 함
- `--json`, `--field` 같은 자동화 옵션의 계약이 고정돼야 함

### 4. Live integration 검증

- `node`, `data`, `webhook`, `stream` 각각 대표 흐름이 실제 Nodit 환경에서 검증돼야 함
- happy path뿐 아니라 인증 실패, 잘못된 입력, 서비스 오류도 테스트돼야 함
- 릴리스 전에 smoke test가 아닌 실제 호출 검증이 돌아야 함

unit test만으로는 공식 CLI 기준을 충족했다고 보기 어렵습니다.

### 5. 릴리스 파이프라인

- 멀티플랫폼 아티팩트가 자동으로 빌드돼야 함
- checksum 또는 무결성 검증 수단이 있어야 함
- changelog가 릴리스마다 정리돼야 함
- 태그와 아티팩트 이름 규칙이 고정돼 있어야 함

### 6. 보안 기준

- API key가 로그, 오류 출력, 디버그 메시지에 노출되면 안 됨
- 예제 문서에 민감정보를 넣지 않아야 함
- dependency/license 점검이 릴리스 전 자동화돼야 함
- 인증 관련 실패 메시지는 안전해야 함

### 7. 지원 운영 기준

- known limitations 문서가 있어야 함
- issue template과 bug report에 필요한 재현 정보가 정의돼 있어야 함
- 사용자가 어떤 범위까지 지원받을 수 있는지 명확해야 함

## Should-have

정식 출시를 위해 강하게 권장되는 항목입니다.

### 1. 체인별 핵심 write 경로 정리

- EVM, Aptos, Sui, Solana의 대표 submit/simulate/send 흐름이 helper 또는 stable raw contract로 문서화돼야 함
- 단, 모든 endpoint를 typed로 만들 필요는 없음
- 자주 쓰는 경로만 helper로 올리고 나머지는 raw contract를 명확히 유지하면 됨

### 2. Stream 운영 기준 확정

- 인증 방식이 문서와 코드에서 일치해야 함
- reconnect/resume 정책이 정의돼 있어야 함
- close code, timeout, message limit 같은 동작이 예측 가능해야 함

### 3. 멀티플랫폼 smoke test

- Windows 포함 설치 후 `--help`와 대표 명령 실행까지 자동 검증
- shell 환경 차이로 인한 설치 실패를 릴리스 전에 잡을 수 있어야 함

### 4. 문서와 실행 예제 일치

- README 예제는 실제로 동작하거나 최소한 smoke 검증을 거쳐야 함
- 플래그명, 응답 예시, 인증 방법이 코드와 어긋나면 안 됨

### 5. 버전 정책

- breaking change 기준이 있어야 함
- deprecation 안내 방식이 있어야 함
- 사용자에게 업그레이드 영향도를 설명할 수 있어야 함

### 6. 운영 편의 기능

- `--verbose` 또는 debug log
- config doctor
- 환경 점검용 self-check
- request id 또는 trace용 출력

## Later

정식 출시 후에도 천천히 확장 가능한 항목입니다.

- 체인별 helper coverage 확대
- preset/profile 저장
- shell completion
- richer config workflow
- 예제 자동 생성
- 더 정교한 UX polish

## 출시 게이트

실무적으로는 아래 기준으로 단계 구분을 권장합니다.

### Developer Preview로 공개 가능

- 기본 설치 가능
- 인증/실행 흐름이 대체로 동작
- `raw` escape hatch 존재
- 아직 지원 범위와 품질이 자주 바뀔 수 있음

### Beta로 승격 가능

- Must-have 대부분 충족
- live integration test 존재
- 대표 체인의 핵심 read/write 흐름 검증 완료
- 문서와 동작이 대체로 일치

### GA로 승격 가능

- Must-have 전부 충족
- Should-have 대부분 충족
- 최근 릴리스에서 심각한 회귀 없음
- 지원 정책과 운영 체계가 준비됨

## `nodit-cli` 기준 현재 우선순위

이 프로젝트에 바로 적용하면 우선순위는 아래가 적절합니다.

1. 지원 매트릭스 문서 작성
2. live integration CI 추가
3. Stream 인증 및 운영 동작 확정
4. 설치/릴리스 검증 자동화
5. 체인별 핵심 write flow 정리
6. 문서와 실제 동작의 정합성 검증

## 판단 기준

공식 CLI는 `모든 API를 다 지원한다`고 말할 수 있을 때가 아니라,
`지원한다고 말한 범위에 대해 회사가 책임질 수 있을 때` 출시해야 합니다.
