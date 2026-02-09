### How to build:
This is described in Zela documentation. Basic steps for this are connecting github repo and registering procedure to the given project and designating branch to which will Zela hook react. Any push to designated branch will trigger build on Zela's dashboard. 
### How to run procedure:
1. Set `ZELA_PROJECT_KEY_ID`:
```sh
export ZELA_PROJECT_KEY_ID=rpk-48eab239d0204704ab787b27a8a1fedf-54c904ac064b4711bca1996d097baa42
```
2. Set `ZELA_PROJECT_KEY_SECRET`. This value will be provided privately. 
```sh
export ZELA_PROJECT_KEY_SECRET={{secret}}
```
3. Install `jq` if not present in your environment.  
MAC example: 
```sh
brew install jq
```
4. Run procedure: 
```sh
    ./run-procedure.sh leader_locator#5915c32a9d1d578f91b1414024d324b21952adfc null
```
where `5915c32a9d1d578f91b1414024d324b21952adfc`is latest commit to main branch before updating this `Readme` file. If you do changes to main branch, you can update the commit hash. 
### Example Response from procedure
```json
{
    "jsonrpc":"2.0",
    "id":1,
    "result": {
        "closest_region":"New York",
        "leader":"9jxgosAfHgHzwnxsHw4RAZYaLVokMbnYtmiZBreynGFP",
        "leader_geo":"North America",
        "slot":398932062
    }
}
```
### Feedback
Script examples in Zela's documentation were not precise. Copy paste did not work, I had to fix the scripts though that could be difference between `zsh` and `bash`. I would suggest adding working `zsh` scripts. Unfortunately I do not have more to add as crypto area is new to me. Code in `RpcClient` was readable and I could find desired methods with ease.

