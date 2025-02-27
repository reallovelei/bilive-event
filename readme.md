# bilive-danmaku
这个库提供模拟bilibili直播的wss连接的功能，目前还在开发中

关于发送弹幕等主动api，可以看我这个仓库: https://github.com/4t145/bilibili-client

## 使用
### 通过websocket
通过使用 https://github.com/4t145/rudanmaku-core

这使你可以通过ws来获取事件，计划在未来支持ipc通讯（uds for linux，命名管道 for windows）

### 作为库使用
因为使用了尚未稳定的`split_array`，所以需要切换到nightly版本
```
rustup override set nightly
```
在`Cargo.toml`中加入
```toml
bilive-danmaku = { git = "https://github.com/4t145/bilive-danmaku", branch = "master" }
tokio = "^1.24.1"
```
使用
```rust
use bilive_danmaku::Connector;
use futures_util::StreamExt;

fn handle_evt(evt: Event) {
    match evt.data {
        Event::PopularityUpdate { popularity } => {
            println!("当前人气值：{popularity}")
        },
        _ => {}
    }
}

async fn tokio_main() {
    let connector = Connector::init(851181).await.unwrap();
    let mut stream = connector.connect().await.unwrap();
    while let Some(maybe_evt) = stream.next().await {
        match maybe_evt {
            Ok(evt) => {
                handle_evt(evt);
            },
            Err(e) => {
                dbg!(e);
            },
        }
    }
    stream.abort();
}
```

数据类型在`model`模块中, 事件类型在`event`模块中
```rust
use model::{User, FansMedal};
use event::Event as BiliEvent;
```
## 已经支持的事件


```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "tag", content="data")]
pub enum EventData {
    Danmaku {
        flag: u64,
        message: DanmakuMessage,
        user: User,
        fans_medal: Option<FansMedal>
    },
    EnterRoom {
        user: User,
        fans_medal: Option<FansMedal>
    },
    Gift {
        user: User,
        fans_medal: Option<FansMedal>,
        gift: Gift,
    },
    GuardBuy {
        level: u64,
        price: u64,
        user: User
    },
    SuperChat {
        user: User,
        fans_medal: Option<FansMedal>,
        price: u64, 
        message: String,
        message_jpn: Option<String>
    },
    WatchedUpdate {
        num: u64
    },
    PopularityUpdate {
        popularity: u32,
    },
    GuardEnterRoom {
        user: User,
    },
    HotRankChanged {
        area: String,
        rank: u64,
        description: String,
    },
    HotRankSettlement {
        uname: String,
        face: String,
        area: String,
        rank: u64,
    },
}
```
可参考：
- [命令原始数据](./src//tests/mock/cmd/)
- [源文件](./src/event.rs)

## feature flag
|flag|功能|
|:---:|:--:|
|`rt_tokio`|使用tokio连接直播间，默认启用|
|`rt_wasm`|运行在wasm直播间|
|`event`|只启用model和event，不包含连接|
|`bincode`|启用bincode正反序列化|
|`json`|启用json正反序列化|
|`verbose`|debug用，输出每条解析的json|
|`debug`|debug用，输出解析错误|


默认只启用`rt_tokio`
比如你想把收到的消息序列化为json格式，启用
```toml
[dependencies.bilive-danmaku]
# ****
features = ["rt_tokio", "json"]
```

## JavaScript/TypeScript 支持
```bash
npm install bilive-danmaku-json@0.1.0-rc5
```
### 使用例
```TypeScript
import {Event, DanmakuEvent} from 'bilive-danmaku-json';
function on_danmaku(data: DanmakuEvent['data']) {
    if(data.junk_flag===0) {
        console.log(data.message);
    }
    // ...
}
// ... 获取data
const evt = JSON.parse(data);
if(evt.tag === 'Danmaku') {
    on_danmaku(evt.data);
}
```
