# SYSTRAY-RS IS ALIVE

To anyone who wants to fork: You can, but I would really recommend against it. Use this repo as reference if you want to, but most of this code was written in 2016, when I was very new to Rust. It doesn't handle cross-platform GUI needs well at all.

If you're going to build your own version of this: Great! If/when you finish it, get in touch and I'll add a link here. But a word of advice: You really do not want to tackle this from the perspective of "Oh I'll do it on [insert prefered OS here] then other people will contribute other platforms". You need to plan for Win/macOS/Linux at the same time or you will end up with an unworkable mess. There are systray implementations in other languages (some of which are listed in the issues here), I'd definitely recommend cribbing from those, much as I tried to crib this one from Go.

I'm going to leave the issues and PRs up since they may also contain relevant reference info for whoever decides to take this on next.

Good luck.

# systray-rs-ty

[![Crates.io](https://img.shields.io/crates/v/systray-ti)](https://crates.io/crates/systray-ti) [![Crates.io](https://img.shields.io/crates/d/systray-ti)](https://crates.io/crates/systray-ti)

[![Build Status](https://travis-ci.org/qdot/systray-rs.svg?branch=master)](https://travis-ci.org/qdot/systray-rs) [![Build status](https://ci.appveyor.com/api/projects/status/lhqm3lucb5w5559b?svg=true)](https://ci.appveyor.com/project/qdot/systray-rs)

systray-rs is a Rust library that makes it easy for applications to
have minimal UI in a platform specific way. It wraps the platform
specific calls required to show an icon in the system tray, as well as
add menu entries.

systray-rs is heavily influenced by
[the systray library for the Go Language](https://github.com/getlantern/systray).

systray-rs currently supports:

- Linux GTK
- Win32

Cocoa core still needed!

# License

systray-rs includes some code
from [winapi-rs, by retep998](https://github.com/retep998/winapi-rs).
This code is covered under the MIT license. This code will be removed
once winapi-rs has a 0.3 crate available.

systray-rs-ty is BSD licensed.

    Copyright (c) 2016-2020, Nonpolynomial Labs, LLC
    All rights reserved.
    
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
    
    * Redistributions of source code must retain the above copyright notice, this
      list of conditions and the following disclaimer.
    
    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.
    
    * Neither the name of the project nor the names of its
      contributors may be used to endorse or promote products derived from
      this software without specific prior written permission.
    
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
    OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

