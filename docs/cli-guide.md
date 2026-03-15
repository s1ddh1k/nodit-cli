# CLI 가이드

## 개요

`nodit-cli`는 Nodit의 공개 개발자 표면을 다루기 위한 Rust CLI입니다.

현재 기준으로는 다음 제품군을 우선 지원합니다.

- `node`: 체인 노드 API
- `data`: Web3 Data API
- `webhook`: Webhook API
- `stream`: Stream API

핵심 방향은 간단합니다.

- 공식 지원의 중심은 stable `raw` contract
- 자주 쓰는 흐름만 `typed helper`로 제공
- write는 `signed payload submit`과 시뮬레이션 위주로만 helper를 둠

## 현재 지원 상태

이 CLI는 현재 `raw-first` + `curated helper` 모델입니다.

- EVM: 조회 helper와 `raw`, `batch`, 일부 submit/simulate helper 제공
- Aptos: 조회 helper와 `raw`, 일부 submit/simulate helper 제공
- Sui: 기본 조회 helper와 `raw`, 일부 execute/simulate helper 제공
- Solana: 주요 조회 helper와 `raw`, 일부 simulate helper 제공
- Web3 Data: 공개 문서 기준 주요 helper 제공
- Webhook / Stream: 기본 관리 및 구독 흐름 제공

반면 helper coverage는 의도적으로 선별적입니다.

- helper가 없는 endpoint 다수
- 노드 보관 키나 지갑 기능이 필요한 고수준 write
- reconnect는 가능하지만 resume이 필요한 Stream 운영

즉, helper가 없는 write/read endpoint는 `raw`로 호출하는 것이 기본 경로입니다.

## 설정

주요 환경 변수:

```bash
export NODIT_API_KEY=your_api_key
export NODIT_API_BASE_URL=https://web3.nodit.io
export NODIT_RPC_URL=https://web3.nodit.io
export NODIT_STREAM_URL=wss://web3.nodit.io
export NODIT_APTOS_API_BASE_URL=https://aptos-mainnet.nodit.io/v1
```

설정 우선순위:

1. CLI 플래그
2. 프로세스 환경 변수
3. 로컬 `.env`
4. `~/.config/nodit-cli/config.toml` (WSL/Linux/macOS)
5. `%AppData%\\nodit-cli\\config.toml` (Windows PowerShell)
6. 기본값

예시 파일:

- [config.example.toml](../config.example.toml)
- [.env.example](../.env.example)

## 명령 구조

### `node`

체인 패밀리 기준으로 구성됩니다.

- `node evm`
- `node aptos`
- `node solana`
- `node sui`
- `node bitcoin`
- `node dogecoin`
- `node xrpl`

예시:

```bash
nodit-cli node evm block-number \
  --protocol ethereum \
  --network mainnet
```

```bash
nodit-cli node aptos ledger-info
```

```bash
nodit-cli node sui chain-identifier
```

### `data`

도메인 기준으로 구성됩니다.

- `data native`
- `data account`
- `data tx`
- `data block`
- `data token`
- `data nft`
- `data ens`
- `data stats`
- `data asset`
- `data multichain`

예시:

```bash
nodit-cli data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

```bash
nodit-cli data tx by-version \
  --protocol aptos \
  --network mainnet \
  --version 1
```

### `webhook`

Webhook 관리와 로컬 수신기를 제공합니다.

- `list`
- `get`
- `create`
- `update`
- `delete`
- `history`
- `serve`

예를 들어 이력 조회는 query parameter 기반 필터를 그대로 노출합니다.

```bash
nodit-cli webhook history \
  --protocol ethereum \
  --network mainnet \
  --subscription-id 12345 \
  --page 1 \
  --rpp 20 \
  --status success \
  --with-event-message true
```

### `stream`

Nodit 이벤트 모델 기준의 구독 흐름을 제공합니다.

- Socket.IO 기반 WebSocket 연결
- typed subscribe
- raw subscribe fallback

참고:

- CLI는 `/v1/websocket` endpoint로 정규화해 연결합니다.
- `BLOCK_PERIOD` 같은 이벤트는 첫 메시지까지 수십 초가 걸릴 수 있습니다.
- 연결이 끊기면 기존 구독을 이어받는 것이 아니라 다시 구독합니다.
- 이때 `subscriptionId`는 유지되지 않고 새 값이 발급됩니다.
- 즉, 자동 reconnect는 가능해도 resume은 지원하지 않는 것으로 봐야 합니다.

## Raw 우선, Typed helper

원칙은 다음과 같습니다.

- `raw`는 보조 수단이 아니라 공식 지원 중심입니다.
- `typed`는 자주 쓰는 경로를 덜 실수하게 만드는 helper입니다.
- 원하는 helper가 없으면 `raw`를 사용하면 됩니다.

예를 들어 자주 쓰는 조회/시뮬레이션은 helper로 바로 호출할 수 있습니다.

```bash
nodit-cli node evm transaction-receipt \
  --protocol ethereum \
  --network mainnet \
  --hash 0xYOUR_TX_HASH
```

```bash
nodit-cli node evm estimate-gas \
  --protocol ethereum \
  --network mainnet \
  --to 0x0000000000000000000000000000000000000000 \
  --data 0x70a082310000000000000000000000000000000000000000000000000000000000000001
```

```bash
nodit-cli node aptos simulate-transaction \
  --estimate-gas-price true \
  --body '{"sender":"0x1","sequence_number":"0","max_gas_amount":"2000","gas_unit_price":"100","expiration_timestamp_secs":"1735689600","payload":{"type":"entry_function_payload","function":"0x1::aptos_account::transfer","type_arguments":[],"arguments":["0x2","1"]},"signature":{"type":"ed25519_signature","public_key":"0x00","signature":"0x00"},"replay_protection_nonce":"0"}'
```

```bash
nodit-cli node solana simulate-transaction \
  --protocol solana \
  --network mainnet \
  --transaction AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAA==
```

```bash
nodit-cli node sui dry-run-transaction-block \
  --protocol sui \
  --network mainnet \
  --tx-bytes AAAC...
```

```bash
nodit-cli node sui execute-transaction-block \
  --protocol sui \
  --network mainnet \
  --tx-bytes AAAC... \
  --signature AKD4... \
  --request-type wait-for-local-execution
```

반면 helper가 없는 메서드는 `raw`가 기본 경로입니다.

```bash
nodit-cli node evm raw \
  --protocol ethereum \
  --network mainnet \
  --method eth_blockNumber \
  --params '[]'
```

Aptos와 Sui도 같은 방식입니다.

```bash
nodit-cli node aptos raw \
  --path /transactions/by_hash/0xHASH
```

```bash
nodit-cli node sui raw \
  --protocol sui \
  --network mainnet \
  --method suix_getReferenceGasPrice \
  --params '[]'
```

## 자주 쓰는 예시

EVM 체인 ID 조회:

```bash
nodit-cli node evm chain-id \
  --protocol ethereum \
  --network mainnet
```

Aptos 이벤트 조회:

```bash
nodit-cli node aptos events-by-creation-number \
  --address 0x1 \
  --creation-number 0 \
  --limit 10
```

Sui 객체 조회:

```bash
nodit-cli node sui object \
  --object-id 0xOBJECT_ID
```

Web3 Data 토큰 페어 조회:

```bash
nodit-cli data token pair-by-asset-type \
  --protocol aptos \
  --network mainnet \
  --asset-type 0x1::aptos_coin::AptosCoin
```

머신 친화 출력:

```bash
nodit-cli --json --field result data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

## 출력 형식

`--json`을 사용하면 다른 도구가 바로 파싱할 수 있는 고정 envelope를 출력합니다.

성공:

```json
{"ok":true,"data":{...}}
```

실패:

```json
{"ok":false,"error":{...}}
```

자주 쓰는 `--field` 별칭:

- `result`
- `body`
- `headers`
- `status`
- `error`

## 제한 사항

현재 문서상 중요하게 알고 있어야 할 제한은 다음과 같습니다.

- write 흐름은 아직 typed UX가 부족함
- 일부 최신 또는 덜 자주 쓰는 API는 `raw` 호출이 필요함
- 체인별 지원 폭은 동일하지 않음
- Nodit의 실제 제품 표면이 바뀌면 command 구성이 같이 바뀔 수 있음

남은 작업은 [docs/roadmap.md](roadmap.md)에 정리합니다.
