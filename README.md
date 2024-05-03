# Mythic Agent Dummy Repository

This repository serves as a placeholder for a fictional Mythic Agent. The Mythic C2 server requires a specific folder structure for its agents, facilitating installation on both the Mythic CLI and GUI.

I conducted extensive research on the structure and deployment process. However, [this link](https://github.com/MythicMeta/Mythic_External_Agent) proved to be immensely helpful in achieving my objectives.

I have worked with the Mythic 3.0 version closely, and installing the test_agent repository using the CLI and GUI became simple after some error handling and perfecting the structure. 

## How to use this repository

This is a simple template for what _your_ git-based repository of _your_ agent should look like. Just fork this repo or download the .zip file and push this structure in your agent's Git repo, make sure to update it with your agent's information. After the setup, you can use the corresponding install command from within the Mythic-cli to install this agent. The Mythic install script for 3rd party agents should work for any Git-based repository (GitHub, GitLab, Bitbucket, etc).

```
sudo ./mythic-cli install github <your agent's url> [optional branch name]
```

## Pre-requisites

Ensure that Mythic is installed (preferably) on your virtual machine. Follow [this link](https://docs.mythic-c2.net/installation) of Mythic's official documentation for the setup if you haven't yet. 

Please make sure you also have Docker and Docker-compose installed on your system because Mythic is entirely deployed on Docker. 

Before installing your agent onto the CLI, ensure that it is up and running. Refer to the official Documentation for the same. 

## Note 

This repo contains pretty much every file you need to build a basic agent and make it installable. 
I have also written guidelines inside a few files about what they do and how you can make them too, can go through them in case needed. 





