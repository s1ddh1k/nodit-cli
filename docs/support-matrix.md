# 지원 매트릭스

이 문서는 현재 `nodit-cli`의 지원 범위를 정리합니다.

상태 값은 아래처럼 해석합니다.

- `supported`: 공식 지원 범위. stable raw contract 또는 curated helper가 문서와 검증 대상에 포함
- `experimental`: first-class command는 있으나 운영 기준이나 검증 범위가 아직 좁음
- `raw-only`: 전용 helper는 없지만 raw contract가 공식 경로로 제공됨
- `not supported`: 현재 CLI에서 직접 지원하지 않음

## 지원 원칙

이 프로젝트는 `raw-first`를 기본 원칙으로 둡니다.

- helper가 없는 것은 곧바로 미지원이라는 뜻이 아님
- 자주 쓰는 흐름만 helper로 고정하고 나머지는 raw contract로 남김
- write helper는 서명된 payload 제출과 simulate 위주로만 선별 제공
- high-level wallet 기능은 지원 범위에 넣지 않음

## 제품 범위

| 영역 | 상태 | 비고 |
|---|---|---|
| Node API | supported | raw contract 중심, 자주 쓰는 흐름만 helper 제공 |
| Web3 Data API | supported | helper 중심이지만 응답/계약은 raw JSON 기준 |
| Webhook API | supported | 관리 기능 제공, 로컬 수신기 포함 |
| Stream API | experimental | 기본 subscribe는 live smoke로 검증됐지만, 재연결은 새 구독 생성 기준이고 resume은 지원되지 않음 |

## 체인별 Node 지원

### EVM 계열

| 범주 | 상태 | 비고 |
|---|---|---|
| 대표 read helper | supported | block number, chain id, balance, tx 조회 등 |
| 대표 write helper | supported | `send-raw-transaction`, `estimate-gas`, `fee-history` |
| batch JSON-RPC | supported | `node evm batch` |
| trace/debug/filter 계열 전체 | raw-only | 일부 자주 쓰는 메서드만 typed 지원 |
| 노드 보관 키 기반 서명 | not supported | 서명은 client-side 전제 |

### Aptos

| 범주 | 상태 | 비고 |
|---|---|---|
| 대표 read helper | supported | ledger/account/resource/events/tx/block/view |
| submit/simulate | supported | encode, submit, simulate, batch submit, wait by hash |
| raw REST | supported | `node aptos raw` |
| BCS/raw table item 세부 표면 | raw-only | 일부 low-level endpoint는 아직 typed 미지원 |
| 노드 보관 키 기반 서명 | not supported | signature 포함 body 제출 전제 |

### Solana

| 범주 | 상태 | 비고 |
|---|---|---|
| 대표 read helper | supported | slot, latest blockhash, block, tx, signature status 등 |
| simulate | supported | `simulate-transaction` |
| send transaction | raw-only | `sendTransaction` 전용 typed command는 아직 없음 |
| 계정/프로그램 조회 확장 | raw-only | `getProgramAccounts`, `getTokenAccountsByOwner` 등 다수 미지원 |
| 노드 보관 키 기반 서명 | not supported | 서명된 tx 전송만 고려 |

### Sui

| 범주 | 상태 | 비고 |
|---|---|---|
| 기본 read helper | supported | chain identifier, reference gas price, object, transaction |
| execute/simulate | supported | dry-run, dev-inspect, execute transaction block |
| `unsafe_*` transaction builder | raw-only | 아직 typed helper 미구현 |
| 조회 확장 표면 | raw-only | checkpoint/query/stake/name-service 등 다수 미지원 |
| 노드 보관 키 기반 서명 | not supported | tx bytes + signature 제출 전제 |

### Bitcoin / Dogecoin / XRPL

| 범주 | 상태 | 비고 |
|---|---|---|
| Bitcoin 기본 helper | supported | block count/hash/block/transaction |
| Dogecoin | raw-only | generic raw RPC만 제공 |
| XRPL | raw-only | generic raw RPC만 제공 |

## 운영 기준

현재 기준으로 공식 지원 범위를 말하려면 아래 조건이 필요합니다.

- 문서와 실제 명령 인터페이스가 일치해야 함
- 인증 실패 시 다음 행동이 명확해야 함
- 대표 흐름은 live smoke 검증이 가능해야 함
- helper 유무와 지원 여부를 같은 의미로 취급하지 않아야 함
- `raw-only` 범위도 stable raw contract라면 공식 지원 범위에 포함할 수 있어야 함

## 무료 플랜 검증 원칙

무료 플랜 사용자는 전체 체인/네트워크를 다 검증할 수 없습니다.

따라서 local smoke와 문서에서는 아래 원칙을 따릅니다.

- 무료 플랜에서 호출 가능한 체인을 우선 검증
- 플랜 제한으로 실패한 항목은 `unsupported`가 아니라 `plan-limited`로 기록
- helper 존재 여부와 실제 플랜 접근 가능 여부를 분리해 문서화

## 로컬 smoke 검증 스냅샷

`2026-03-15` 기준 로컬 smoke에서 아래 항목은 실제 호출이 통과했습니다.

- EVM `block-number`
- Aptos `ledger-info`
- Solana `slot`
- Sui `chain-identifier`
- Data `native balance`
- Stream `BLOCK_PERIOD` 구독 1회 수신

Stream에 대해서는 아래처럼 해석합니다.

- `reconnect`: 가능하지만 클라이언트가 다시 연결하고 다시 `subscription`을 보내는 방식
- `resume`: 미지원
- 재구독 시 `subscriptionId`는 유지되지 않으며 새 값이 발급됨
- 따라서 끊긴 동안의 이벤트를 무손실로 이어받는 기능은 현재 지원 범위로 보지 않음

Stream smoke는 기본값으로는 실행하지 않습니다.

- 이유: CU 소모와 대기 시간이 커서 기본 smoke에서는 비용을 줄이기 위해 제외
- 실행 방법: `NODIT_INCLUDE_STREAM=1 bash scripts/live-smoke.sh`

## 현재 공백

정식 출시 기준으로 아직 남은 큰 공백은 아래입니다.

- Stream 운영 기준 확정
- 체인별 남은 raw contract 중 핵심 helper 후보 정리
- live integration 검증 확대
- release smoke 자동화
