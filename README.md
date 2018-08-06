# LibSovToken


This is a payment handler library to work with libindy. It may one day be merged into libindy.
## Running the indy pool 

### Build the pool
```
    cd devops/indy-pool/ && docker build -t indy_pool . 
```
### run the poool
``` 
    docker run -itd -p 9701-9708:9701-9708 indy_pool
```
## File Structure
```
.
├── doc
├── libsovtoken
│   ├── src
│   │   ├── api
│   │   ├── logic
│   │   └── utils
│   ├── target
│   └── tests
└── samples
```
