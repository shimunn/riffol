// Copyright (c) 2018, [Ribose Inc](https://www.ribose.com).
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NO/T
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate libc;

#[derive(Debug, Clone)]
pub enum Limit {
    Num(u64),
    Infinity,
}

#[derive(Debug, Clone)]
pub enum RLimit {
    Memory(Limit),
    Procs(Limit),
    Files(Limit),
}

pub fn setlimit(rlimit: &RLimit) {
    let (resource, limit) = match rlimit {
        RLimit::Memory(v) => (libc::RLIMIT_AS, v),
        RLimit::Procs(v) => (libc::RLIMIT_NPROC, v),
        RLimit::Files(v) => (libc::RLIMIT_NOFILE, v),
    };
    let limit = match limit {
        Limit::Num(v) => *v,
        Limit::Infinity => libc::RLIM_INFINITY,
    };
    unsafe {
        let _result = libc::setrlimit64(
            resource,
            &libc::rlimit64 {
                rlim_cur: limit,
                rlim_max: limit,
            },
        );
    };
}