# Pull Request 

## Types of pull request

### Type one: Changes to the codebase

These pull request are changes to any file that deals with logic and function of the plugin. These are usally include but are not limited to: bug fixes, documentation updates, new features, etc.
#### Rules

1. Anyone in the community can make these pull request 
2. The pull request needs to:
    * Pass all tests through jenkins
    * Have at least two approvals from maintainers

### Type two: Dependancy changes 

For now it has been agreed upon that libsovtoken will work on the latest stable builds of it's dependencies. But there can be pull requests if this does not meet a majorities needs.

#### Rules

1. Anyone in the community can make these pull request 
2. The pull request needs to:
    * clear documented explenation of why versions should change
    * Pass all tests through jenkins
    * Have _all_ maintainers approve of the change 


### Type three: Changes to Devops

This includes changes to any files in the devops folder. It is to be noted that the community should not be making changes to these files because they are for internal use only.

#### Rules

1. Only internal entities can make this pull request
2. The pull request needs to:
    * Pass all tests through jenkins
    * Have at least 2 maintainers approve of the change

## Approving changes

### What is the approval process?

1. A maintainer must thoroughly read the changes and consider the effects of the pull request
2. A maintainer must consider and address any concerns left in the comments section
3. A maintainer must check that all tests pass in the jenkins environment
4. A maintainer must test the changes on his local machine and make sure that all tests pass using the ***libsovtoken*** `indy_pool` 
