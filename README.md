## Team Volunteer App
Command line application for 'volunteering' a team member

## Overview
Simple random picker for choosing a team member to volunteer for a task, ie scrumaster or fetch the coffee.

## Features
Abilty to pre specify a list of team members, First and Last name.
- press enter to choose a volunteer
- type index to display the random index.
- type quit to exit from the app

## Configuration
Update the team.json and add new team members with follwing schema
firstName: persons first name - String
lastName: persons last name - String
'''
    {
        "firstName": "Dean",
        "lastName": "Salleyman"
    }
    '''
## Getting started
copy volunteer.zip file to local directory.
unzip archive
update team.json file with team members.
cd into volunteer directory
run by entering ./volunteer

## Build Deploy
Cargo build 
save volunteer and assocated team.json to local directory.



## Release Notes
0.0.1

Initial Release of application.