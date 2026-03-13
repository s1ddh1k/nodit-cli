# nodit-cli

Nodit 서비스를 위한 Rust CLI입니다. Linux, macOS, Windows 네이티브 빌드를 지원합니다.

## 범위

이 프로젝트는 현재 공개 문서로 확인 가능한 Nodit 표면을 대상으로 구성되어 있습니다:

- Elastic Node / JSON-RPC
- Sui Node API
- Web3 Data API
- Webhook API
- Stream API over WebSocket
- Aptos Node API

`Datasquare`, `Dedicated Node`, 콘솔 전용 워크플로는 아직 first-class 명령으로 노출하지 않습니다. 공개 자동화 표면이 제한적이거나 제품별 성격이 강하기 때문입니다. 다만 전송 계층은 충분히 범용적으로 만들어져 있어 API 표면이 명확해지면 나중에 추가할 수 있습니다.

## 현재 지원 모델

현재 CLI는 조회(Read) 중심으로 설계되어 있습니다.

- EVM: 조회 기능이 넓게 갖춰져 있고 `raw`, `batch` escape hatch가 있음
- Aptos: 조회 기능이 넓게 지원되고 `raw` fallback도 있음
- Sui: 일부 조회 기능과 `raw` fallback이 있음
- Web3 Data API: 조회/검색 커버리지가 우선 대상임

프로젝트 전반에서 typed write flow는 아직 제한적입니다.

- EVM의 `eth_sendRawTransaction` 같은 write 메서드는 현재 `node evm raw`로만 호출할 수 있음
- Aptos의 제출/시뮬레이션 계열은 아직 대부분 first-class typed command로 노출되지 않음
- Sui의 트랜잭션 제출 계열도 아직 first-class typed command로 노출되지 않음

체인 상태를 바꾸는 워크플로라면, README나 `--help`에 전용 typed command가 명시돼 있지 않은 이상 `raw-only`이거나 아직 전용 명령으로 모델링되지 않은 상태라고 보면 됩니다.

## 환경 변수

```bash
export NODIT_API_KEY=your_api_key
export NODIT_API_BASE_URL=https://web3.nodit.io
export NODIT_RPC_URL=https://web3.nodit.io
export NODIT_STREAM_URL=wss://web3.nodit.io
export NODIT_APTOS_API_BASE_URL=https://aptos-mainnet.nodit.io/v1
```

체인이나 네트워크별로 더 구체적인 URL이 필요하면 플래그로 덮어쓸 수 있습니다.

설정은 다음 우선순위로 적용됩니다.

1. CLI 플래그
2. 프로세스 환경 변수
3. 로컬 `.env`
4. `~/.config/nodit-cli/config.toml`
5. 기본값

예시 설정 파일:

- [config.example.toml](/home/eugene/git/nodit-cli/config.example.toml)
- [.env.example](/home/eugene/git/nodit-cli/.env.example)

## 자동화 친화 사용법

다른 도구나 에이전트가 출력을 파싱해야 한다면 `--json`을 사용합니다.

```bash
nodit-cli --json data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

필요한 필드만 뽑고 싶다면 `--field`를 함께 사용합니다.

```bash
nodit-cli --json --field result data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

자주 쓰는 필드 별칭:

- `result` -> `data.body.result`
- `body` -> `data.body`
- `headers` -> `data.headers`
- `status` -> `data.status`
- `error` -> `error`

`--output`을 생략하면 인터랙티브 터미널에서는 읽기 쉬운 JSON을, 파이프 출력에서는 compact JSON을 사용합니다. 모든 명령 결과는 같은 envelope를 사용합니다.

```json
{"ok":true,"data":{...}}
```

```json
{"ok":false,"error":{...}}
```

## 설치

macOS를 포함해 한 줄로 설치하려면:

```bash
curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash
```

이 스크립트는 다음을 수행합니다.

- `rustup`이 없으면 자동 설치
- 저장소를 클론한 뒤 `cargo build --release` 실행
- `~/.local/bin/nodit-cli`에 바이너리 설치
- macOS에서는 quarantine attribute 제거

원하는 브랜치나 태그로 설치:

```bash
NODIT_CLI_REF=v0.1.0 curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash
```

설치 경로 변경:

```bash
NODIT_CLI_BIN_DIR="$HOME/bin" curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash
```

## 빌드

```bash
cargo build
```

릴리스 빌드:

```bash
cargo build --release
```

이 저장소에는 다음 GitHub Actions 워크플로가 포함되어 있습니다.

- `ubuntu-latest`, `macos-latest`, `windows-latest` 대상 CI
- 세 운영체제용 태그 릴리스 아티팩트 생성

## 릴리즈

이 저장소의 릴리즈는 Git 태그 푸시로 진행합니다.

1. `Cargo.toml`의 버전을 올립니다.
2. 아래 검증을 로컬에서 실행합니다.

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

3. 변경사항을 커밋합니다.

```bash
git add .
git commit -m "release: v0.1.1"
```

4. 태그를 만들고 원격에 푸시합니다.

```bash
git tag v0.1.1
git push origin main
git push origin v0.1.1
```

`v*` 태그가 푸시되면 GitHub Actions가 자동으로 릴리즈를 만들고 다음 아티팩트를 업로드합니다.

- Linux: `nodit-cli-linux.tar.gz`
- macOS: `nodit-cli-macos.tar.gz`
- Windows: `nodit-cli-windows.zip`

## 문서

- [docs/cli-guide.md](/home/eugene/git/nodit-cli/docs/cli-guide.md): 실제 사용 기준의 CLI 안내
- [docs/roadmap.md](/home/eugene/git/nodit-cli/docs/roadmap.md): 남은 작업과 우선순위
- [docs/nodit-official/README.md](/home/eugene/git/nodit-cli/docs/nodit-official/README.md): 로컬에 저장한 공식 문서 스냅샷 안내

## 명령 예시

### Node API

```bash
nodit-cli node evm block-number \
  --protocol ethereum \
  --network mainnet
```

체인 ID 조회:

```bash
nodit-cli node evm chain-id \
  --protocol ethereum \
  --network mainnet
```

트랜잭션 영수증 조회:

```bash
nodit-cli node evm transaction-receipt \
  --protocol ethereum \
  --network mainnet \
  --hash 0xYOUR_TX_HASH
```

트랜잭션 카운트(nonce) 조회:

```bash
nodit-cli node evm transaction-count \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

특정 주소의 코드 조회:

```bash
nodit-cli node evm code \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Raw JSON-RPC 호출:

```bash
nodit-cli node evm raw \
  --protocol ethereum \
  --network mainnet \
  --method eth_blockNumber \
  --params '[]'
```

Batch JSON-RPC 호출:

```bash
nodit-cli node evm batch \
  --protocol ethereum \
  --network mainnet \
  --body '[{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]},{"jsonrpc":"2.0","id":2,"method":"eth_chainId","params":[]}]'
```

Aptos Node API:

```bash
nodit-cli node aptos ledger-info
```

```bash
nodit-cli node aptos estimate-gas-price
```

```bash
nodit-cli node aptos transaction-by-version \
  --version 1
```

```bash
nodit-cli node aptos block-by-height \
  --block-height 1 \
  --with-transactions false
```

```bash
nodit-cli node aptos view \
  --function 0x1::coin::balance \
  --type-arg 0x1::aptos_coin::AptosCoin \
  --arguments-json '[\"0x1\"]'
```

```bash
nodit-cli node aptos events-by-creation-number \
  --address 0x1 \
  --creation-number 0 \
  --limit 10
```

Solana Node API:

```bash
nodit-cli node solana slot \
  --protocol solana \
  --network mainnet
```

```bash
nodit-cli node solana latest-blockhash \
  --protocol solana \
  --network mainnet
```

```bash
nodit-cli node solana block-height \
  --protocol solana \
  --network mainnet
```

```bash
nodit-cli node solana version \
  --protocol solana \
  --network mainnet
```

```bash
nodit-cli node solana epoch-info \
  --protocol solana \
  --network mainnet
```

```bash
nodit-cli node solana minimum-balance-for-rent-exemption \
  --protocol solana \
  --network mainnet \
  --data-length 165
```

Sui Node API raw JSON-RPC 예시:

```bash
nodit-cli --json node sui raw \
  --protocol sui \
  --network mainnet \
  --method suix_getReferenceGasPrice \
  --params '[]'
```

Sui 체인 식별자 조회:

```bash
nodit-cli --json node sui chain-identifier \
  --protocol sui \
  --network mainnet
```

Sui 객체 조회:

```bash
nodit-cli --json node sui object \
  --protocol sui \
  --network mainnet \
  --object-id 0x53e4567ccafa5f36ce84c80aa8bc9be64e0d5ae796884274aef3005ae6733809
```

### Web3 Data API

```bash
nodit-cli data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

토큰 홀더 조회:

```bash
nodit-cli data token holders-by-contract \
  --protocol ethereum \
  --network mainnet \
  --contract 0x0000000000000000000000000000000000000000
```

계정 기준 NFT 전송 조회:

```bash
nodit-cli data nft transfers-by-account \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

주소 기준 ENS 이름 조회:

```bash
nodit-cli data ens name-by-address \
  --protocol ethereum \
  --network mainnet \
  --address 0x0000000000000000000000000000000000000000
```

계정 통계 조회:

```bash
nodit-cli data stats account \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Aptos 버전 기준 트랜잭션 조회:

```bash
nodit-cli data tx by-version \
  --protocol aptos \
  --network mainnet \
  --version 1
```

Aptos asset type 기준 토큰 페어 조회:

```bash
nodit-cli data token pair-by-asset-type \
  --protocol aptos \
  --network mainnet \
  --asset-type 0x1::aptos_coin::AptosCoin
```

Web3 Data API로 Bitcoin 계정 정보 조회:

```bash
nodit-cli data account total-transaction-count \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

```bash
nodit-cli data account unspent-transaction-outputs \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

```bash
nodit-cli data tx by-account \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

XRPL ledger 해시 또는 인덱스 조회:

```bash
nodit-cli data block ledger-by-hash-or-index \
  --protocol xrpl \
  --network mainnet \
  --ledger 95000002
```

XRPL ledger 내 트랜잭션 조회:

```bash
nodit-cli data tx in-ledger \
  --protocol xrpl \
  --network mainnet \
  --ledger 95000002
```

XRPL currency와 issuer 기준 토큰 전송 조회:

```bash
nodit-cli data token transfers-by-currency-and-issuer \
  --protocol xrpl \
  --network mainnet \
  --currency USD \
  --issuer-address rIssuerAddressHere
```

Bitcoin의 block 계열 조회는 EVM과 같은 경로 형태라고 가정하지 않습니다. 공식 문서 기준 매핑이 더 정리되기 전까지는 검증된 `data account`와 `data tx` 흐름 위주로 사용하는 편이 안전합니다.

검증된 Bitcoin 블록 조회:

```bash
nodit-cli data block by-hash-or-number \
  --protocol bitcoin \
  --network mainnet \
  --block 0
```

### Webhook 및 Stream

`BLOCK_PERIOD`용 Webhook:

```bash
nodit-cli webhook create \
  --protocol ethereum \
  --network mainnet \
  --event-type BLOCK_PERIOD \
  --webhook-url https://example.com/hook \
  --period 1
```

`ADDRESS_ACTIVITY`용 Webhook:

```bash
nodit-cli webhook create \
  --protocol ethereum \
  --network mainnet \
  --event-type ADDRESS_ACTIVITY \
  --webhook-url https://example.com/hook \
  --address 0xdAC17F958D2ee523a2206206994597C13D831ec7,0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
```

`MINED_TRANSACTION`용 Stream:

```bash
nodit-cli stream \
  --protocol ethereum \
  --network mainnet \
  --event-type MINED_TRANSACTION \
  --address 0xc1f18552bD0dc6D4f5A2dccc756F10C4882E2F4A \
  --messages 3
```

## 설계 메모

- `node`와 `data`를 가장 중요한 표면으로 둡니다.
- 제품 단위 명령은 CLI 계층에서 분리합니다.
- `node`는 `evm`, `aptos` 같은 체인 패밀리 기준으로 묶습니다.
- `data`는 `native`, `account`, `tx`, `block`, `token`, `nft`, `ens`, `stats`, `asset`, `multichain` 같은 도메인 기준으로 묶습니다.
- 현재 `solana`는 node 표면에서, `bitcoin`은 data 표면에서 실검증이 진행된 상태입니다.
- 안정적인 typed command가 아직 없는 곳에는 `raw` 명령을 유지합니다.
- `webhook serve`는 Nodit 콜백을 로컬에서 받는 용도로 쓸 수 있고 `signingKey`로 Nodit `x-signature`를 검증할 수 있습니다.
- 체인별 스키마를 너무 일찍 고정하지 않기 위해 응답은 raw JSON 중심으로 유지합니다.
- Linux, macOS, Windows 대상 네이티브 CI와 릴리스 패키징이 설정돼 있습니다.
