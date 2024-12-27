# Metamask адреса

Адреса из метамаска определяются в node/src/chain_spec.rs в функции testnet_genesis, поле evm, поле balance задает баланс для этого адреса.
При копировании из метамаска адреса уже в нужном формате, необходимо только убрать "0x".



# Как поднять ноду локально

1) Првести в соответствие версию rust с помощью уоманд:
  rustup install nightly-2023-02-01;
  rustup override set nightly-2023-02-01;
  rustup target add wasm32-unknown-unknown --toolchain nightly-2023-02-01;

2) Выполнить cargo build --release

3) Выполнить:

  ./target/release/node-template purge-chain --base-path ./tmp/node01 --chain local_testnet;
  ./target/release/node-template build-spec --disable-default-bootnode --chain local_testnet > customSpec.json;
  ./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json;

4) Если нужно заинсёртит друние ключи, это делается с помощью следующих команд:
   для aura ./target/release/node-template key insert  --base-path ./tmp/node01  --chain customSpecRaw.json  --scheme Sr25519  --suri "<сури>"  --key-type "aura";
   для grandpa ./target/release/node-template key insert  --base-path ./tmp/node01  --chain customSpecRaw.json  --scheme Ed25519  --suri "сури"  --key-type "gran";

5) Запустить ноду командой:
./target/release/node-template \
  --base-path ./tmp/node01 \
  --keystore-path "./tmp/node01/chains/local_testnet/keystore" \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9943 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \
  --public-addr /ip4/127.0.0.1/tcp/30333 \
  --rpc-cors all 
  --rpc-methods Unsafe 
  --rpc-external \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  --ws-external 




# Подключение Metamask к ноде

Запустив ноду необходимо подключить свою сеть.
-название произвольное;
-url - http://<ip, на котором запущена нода>:<rpc порт ноды>;
-chain id копируется из runtime/src/lib.rs, строка 301;
-символ валюты произвольный


После выполнения этих шагов, балансы зачисляться на адреса метамаск и будет возможность проводить транзакции внутри сети
