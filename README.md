# Metamask адреса

Адреса из метамаска определяются в node/src/chain_spec.rs в функции testnet_genesis, поле evm, поле balance задает баланс для этого адреса.
При копировании из метамаска адреса уже в нужном формате, необходимо только убрать "0x".



# Как поднять ноду локально

1) Првести в соответствие версию rust с помощью команд: 
  rustup install nightly-2023-02-01; 
  rustup override set nightly-2023-02-01; 
  rustup target add wasm32-unknown-unknown --toolchain nightly-2023-02-01;

2) Выполнить cargo build --release

3) Выполнить: 
  ./target/release/node-template purge-chain --base-path ./tmp/node01 --chain local_testnet;
  ./target/release/node-template build-spec --disable-default-bootnode --chain local_testnet > customSpec.json;
  ./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json;

4) Если нужно заинсёртит друние ключи, это делается с помощью следующих команд: 
для aura ./target/release/node-template key insert  --base-path ./tmp/node01  --chain customSpecRaw.json  --scheme Sr25519  --suri "live smart tape caught sell decline knee file average accuse syrup family"  --key-type "aura"; 
для grandpa ./target/release/node-template key insert  --base-path ./tmp/node01  --chain customSpecRaw.json  --scheme Ed25519  --suri "live smart tape caught sell decline knee file average accuse syrup family"  --key-type "gran";

Примеры ключей 
Secret phrase:       live smart tape caught sell decline knee file average accuse syrup family 
  Network ID:        substrate 
  Secret seed:       0xbe245762c720569aea48f1976bd937f5b3c14b2d41e3857b23e1c8d6e89bca6c 
  Public key (hex):  0xa08dcd654b785edbacf3de8a81022ec1d79ee4e9e226eaf8c3e0177370380256 
  Account ID:        0xa08dcd654b785edbacf3de8a81022ec1d79ee4e9e226eaf8c3e0177370380256 
  Public key (SS58): 5FhDeEeEVFJbrEcFKPGw8DG1eSStMYDgzWJhk6tgGHK2QSsb 
  SS58 Address:      5FhDeEeEVFJbrEcFKPGw8DG1eSStMYDgzWJhk6tgGHK2QSsb 


Secret phrase:       live smart tape caught sell decline knee file average accuse syrup family 
  Network ID:        substrate 
  Secret seed:       0xbe245762c720569aea48f1976bd937f5b3c14b2d41e3857b23e1c8d6e89bca6c 
  Public key (hex):  0x19d86e38c784910ab80935d6688811858058cc8ab53cd85dcd0b051b6f2c0e09 
  Account ID:        0x19d86e38c784910ab80935d6688811858058cc8ab53cd85dcd0b051b6f2c0e09 
  Public key (SS58): 5CebM7X4h3Fs88q9gJSTQYWwHag8QfxqwN6SobvSKKXp8GFU 
  SS58 Address:      5CebM7X4h3Fs88q9gJSTQYWwHag8QfxqwN6SobvSKKXp8GFU 

6) Запустить ноду командой: 
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
