#!/usr/bin/env python3.11
import mythic_container

#pylint: disable=wildcard-import,unused-wildcard-import

from test_agent.mythic import * 
 
mythic_container.mythic_service.start_and_run_forever()