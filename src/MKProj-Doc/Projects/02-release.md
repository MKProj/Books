# Releases/Version System

> _Any Projects that are specified in a release cycle (actively releases software to Github) will undergo the Version/System discussed below!_


## Release System

In our release system, we will be following the edge -> beta -> stable channel format. 
A new Stable release will always come in 4 weeks at around the 15th of every month. 
Each of these programs will have the following branches to indicate the following channels: 

- main => (Will Contain the Current Stable Release)
- stable => stable channel
- beta => beta channel
- edge => edge channel

So now you may ask what the difference of these may be? Well they correspond to the different stages 
of the applications build process.  

### Edge:
- The everyday changes of the code that is always changing, and may be unstable 
- Once feature may be stable, it will be moved to the beta channel

### Beta: 
- The beta channel is used to organize all the new changes
- The beta channel is pre-production and should be relatively stable during production 

### Stable: 
- The stable channel is the ready to use code
- All the code should be working properly and safely
- Depending on the new feature: 
  - The Major or Minor will be incremented

### Code Names 
Each new release will have an accompanying code name associated with it, like v0.5's Aristotle code name. 
These names will always be named after a famous scientist or influence in science. 

## Version System 

The projects follow the same semantic version system other Rust projects use:


Version M.m.P:  

* M - Major change (Like a new feature)
* m - Minor change (New API)
* P - Patches (Fixes on the code)
