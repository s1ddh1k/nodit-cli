# 로드맵

이 문서는 `nodit-cli`에서 아직 남아 있는 작업만 정리합니다.

현재 기준:

- Web3 Data 조회 커버리지는 넓게 확보됨
- EVM, Aptos, Solana, Sui의 node 조회 기능도 기본 골격이 있음
- Webhook / Stream 기본 흐름은 이미 동작함

남은 큰 작업은 아래와 같습니다.

## 우선순위

### 1. Node write flow

가장 큰 공백입니다.

- EVM typed write helper
  - 예: `eth_sendRawTransaction`
- Aptos typed submission / simulation flow
- Sui typed submission / simulation flow

현재는 대부분 `raw` 호출에 의존하거나, 아직 전용 명령이 없습니다.

### 2. Aptos / Sui read coverage 확대

조회 중심 helper는 많이 늘었지만 아직 빈 곳이 남아 있습니다.

- Aptos 남은 read endpoint 정리
- Sui read helper 추가 확대

### 3. Webhook / Stream 조건 빌더 보강

현재도 기본 사용은 가능하지만, 조건 생성 UX는 더 다듬을 수 있습니다.

- 이벤트 타입별 condition builder
- Aptos 전용 filter
- 재연결 / resume 전략

### 4. 실행 환경 개선

실사용 편의성 보강 작업입니다.

- 체인 / 네트워크 preset
- 저장 프로필
- live integration 검증 확대

## 작업 원칙

- `node`, `data`, `webhook`, `stream` 네 축은 유지
- 자주 쓰는 흐름은 typed command 우선
- 변화가 잦은 영역은 `raw` escape hatch 유지
- 실제 Nodit 동작은 공식 문서와 실호출로 검증
