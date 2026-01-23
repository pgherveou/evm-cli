# User Story: Deploy a Contract Instance
#
# As a developer, I want to deploy a new instance of my contract
# to the blockchain so I can interact with it.

Output tests/recordings/contracts-menu/deploy-instance.ascii

Source ./tests/recordings/common/setup.tape

# STORY: Deploy a new contract instance
# Step 1: Contract is loaded, navigate to "Deploy new instance"
Wait+Screen /Demo/
Type "j"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-deploy-01-select-deploy.png

# Step 2: Press Enter to open parameter popup
Enter
Wait+Screen /constructor|Enter Parameters/
Wait+Screen /Target|EVM/
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-deploy-02-popup-opened.png

# Step 3: Enter constructor parameter (uint256 _initial = 42)
Type "42"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-deploy-03-param-entered.png

# Step 4: Press Enter to deploy
Enter
Sleep 1.5s

# Step 5: Deployment complete - instance appears with address and methods
Wait+Screen /0x[0-9a-fA-F]/
Wait+Screen /increment|getCount/
Screenshot tests/recordings/contracts-menu/story-deploy-04-deployed.png

Ctrl+C
