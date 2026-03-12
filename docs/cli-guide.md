# CLI 가이드

## 개요

`nodit-cli`는 Nodit의 공개 개발자 표면을 다루기 위한 Rust CLI입니다.

현재 기준으로는 다음 제품군을 우선 지원합니다.

- `node`: 체인 노드 API
- `data`: Web3 Data API
- `webhook`: Webhook API
- `stream`: Stream API

핵심 방향은 간단합니다.

- 자주 쓰는 조회 흐름은 typed command로 제공
- 덜 자주 쓰이거나 입력 형태가 자주 바뀌는 영역은 `raw`로 호출 가능
- 체인 상태를 바꾸는 write 흐름은 아직 제한적으로만 지원

## 현재 지원 상태

이 CLI는 현재 `조회(Read) 중심`입니다.

- EVM: 조회형 JSON-RPC helper와 `raw`, `batch` 지원
- Aptos: 조회형 REST helper를 넓게 지원하고 `raw` fallback 제공
- Sui: 기본 조회 helper와 `raw` fallback 제공
- Web3 Data: 공개 문서 기준 주요 조회 endpoint를 폭넓게 커버
- Webhook / Stream: 기본 관리 및 구독 흐름 지원

반면 다음 영역은 아직 본격적으로 정리되지 않았습니다.

- EVM typed write helper
- Aptos typed submission / simulation flow
- Sui typed submission / simulation flow

즉, 트랜잭션 제출이나 상태 변경이 필요한 경우에는 아직 `raw` 호출이 필요하거나, 전용 명령이 없는 상태일 수 있습니다.

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
4. `~/.config/nodit-cli/config.toml`
5. 기본값

예시 파일:

- [config.example.toml](/home/eugene/git/nodit-cli/config.example.toml)
- [.env.example](/home/eugene/git/nodit-cli/.env.example)

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

### `stream`

Nodit 이벤트 모델 기준의 구독 흐름을 제공합니다.

- WebSocket 연결
- typed subscribe
- raw subscribe fallback

## Typed 명령과 Raw 명령

원칙은 다음과 같습니다.

- 먼저 typed command를 확인
- 원하는 기능이 없으면 `raw` 사용

예를 들어 EVM에서 잘 알려진 조회 메서드는 typed helper로 바로 호출할 수 있습니다.

```bash
nodit-cli node evm transaction-receipt \
  --protocol ethereum \
  --network mainnet \
  --hash 0xYOUR_TX_HASH
```

반면 전용 명령이 없는 메서드는 `raw`로 호출합니다.

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

남은 작업은 [docs/roadmap.md](/home/eugene/git/nodit-cli/docs/roadmap.md)에 정리합니다.
