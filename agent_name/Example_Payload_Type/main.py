#!/usr/bin/env python3.11
import mythic_container

#pylint: disable=wildcard-import,unused-wildcard-import

#from agent_name.mythic import * 

#not using the above line as of now since it wll import 
#the functions of the agent in this workspace.
#and currently there are no functions defined for the agent.  

mythic_container.mythic_service.start_and_run_forever()