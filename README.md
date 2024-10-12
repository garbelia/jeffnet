# JEFFnet version 0.1.3

Currently just basically a rust rewrite of Cat Nuke Thing, with a few fun extra features I thought might be handy. 

TODO:
- Nation spec categoriser - DONE V0.1.2
- Setup bash script - DONE V0.1.2
- Userscript
- Containerise stuff
- Any other suggestions, additional features
- Tweak the sheet so it isn't literally an exact copy of cat nuke thing

The repo is only really public so the other tech folk can look at it, but I'm open to taking it private or accepting any pull requests if this is deemed worthy of continuing.


## Installation

### Windows
1. Begin by installing Rust if you haven't already at https://rustup.rs/. Select all default options when prompted by the installer.
2. Either download and extract the repo or clone it.
3. Open a shell in the folder.
4. Run setup.sh twice with .\setup.sh
5. Select y when prompted the first time and n the second time.
6. Move jeffnet.user.js into the Tampermonkey installed scripts tab.

### Linux
You know already, but just to make it easier:
If you don't already have Rust installed -
'''
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
exec bash
'''
Clone the repo -
'''
git clone https://github.com/garbelia/jeffnet.git
cd jeffnet
'''
Run twice and follow the instructions in the script -
'''
./setup.sh
'''

### Mac 
¯\_(ツ)_/¯