# User Story: Deploy a Contract
#
# As a developer, I want to deploy a new instance of my contract
# to the blockchain so I can interact with it.
#
# Acceptance Criteria:
# - AC-CM-10: Instance auto-expands when deployed
# - AC-CM-15: Enter on "Deploy new instance" shows parameter popup
# - AC-CM-16: Parameter popup includes EVM/PVM target selection
# - AC-CM-17: Deployed instance address appears in tree after deployment

Output tests/recordings/user-stories/contract-management/02-deploy-contract.ascii

Source ./tests/recordings/common/setup.tape

# Step 1: Contract is loaded, navigate to "Deploy new instance"
Wait+Screen /Demo/
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/02-deploy-01-select.png

# Step 2: Press Enter to open parameter popup
Enter
Wait+Screen /constructor|Enter Parameters/
Wait+Screen /Target|EVM/
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/02-deploy-02-popup.png

# Step 3: Enter constructor parameter (uint256 _initial = 42)
Type "42"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/02-deploy-03-param.png

# Step 4: Press Enter to deploy
Enter
Sleep 1.5s

# Step 5: Deployment complete - instance appears with address and methods
Wait+Screen /0x[0-9a-fA-F]/
Wait+Screen /increment|getCount/
Screenshot tests/recordings/user-stories/contract-management/02-deploy-04-deployed.png

Ctrl+C
