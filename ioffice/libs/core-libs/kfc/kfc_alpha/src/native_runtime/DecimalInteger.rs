/*
 * Copyright (c) 1994, 2023, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */
// Native Android java.lang.Integer/Long decimal parse adapters. The source
// methods use the same negative-accumulation loop at their respective widths.
// Character.digit(char,10) is copied exhaustively from the original Android
// runtime, retaining UTF-16 char semantics (surrogate halves are not digits).
pub fn characterDigit(c:u16)->i32{let d=include_bytes!("character-decimal.bin")[c as usize];if d==255{-1}else{d as i32}}
macro_rules! decimal_parse {($name:ident,$t:ty)=>{
    // java public static int/long parseInt/parseLong(String s, int radix) throws NumberFormatException {
    pub fn $name(s:&[u16])->Option<$t>{
        // rust: radix=10 overload; Option transports the original NumberFormatException.
        // java boolean negative = false; int i = 0, len = s.length(); int/long limit = -Integer/Long.MAX_VALUE;
        let mut negative=false;let mut i=0;let len=s.len();let mut limit= -<$t>::MAX;
        // java if (len > 0) {
        if len>0{
            // java char firstChar = s.charAt(0); if (firstChar < '0') {
            let firstChar=s[0];if firstChar<48{
                // java if (firstChar == '-') { negative = true; limit = Integer/Long.MIN_VALUE; } else if (firstChar != '+') { throw NumberFormatException.forInputString(s, radix); }
                if firstChar==45{negative=true;limit=<$t>::MIN;}else if firstChar!=43{return None;}
                // java if (len == 1) { throw NumberFormatException.forInputString(s, radix); } i++;
                if len==1{return None;}i+=1;
            }
            // java int/long multmin = limit / radix; int/long result = 0;
            let multmin=limit/10;let mut result:$t=0;
            // java while (i < len) {
            while i<len{
                // java int digit = Character.digit(s.charAt(i++), radix);
                let digit=characterDigit(s[i]);i+=1;
                // java if (digit < 0 || result < multmin) { throw NumberFormatException.forInputString(s, radix); }
                if digit<0||result<multmin{return None;}
                // java result *= radix;
                result*=10;
                // java if (result < limit + digit) { throw NumberFormatException.forInputString(s, radix); }
                if result<limit+digit as $t{return None;}
                // java result -= digit;
                result-=digit as $t;
            }
            // java return negative ? result : -result;
            Some(if negative{result}else{-result})
        // java } else { throw NumberFormatException.forInputString(s, radix); }
        }else{None}
    }
}}
decimal_parse!(parseInt,i32);
decimal_parse!(parseLong,i64);
// java public static int parseInt(String s, int radix) throws NumberFormatException {
// Native Option carries NumberFormatException; UTF-16 Character.digit is retained.
pub fn parseIntRadix(s:&[u16],radix:i32)->Option<i32>{
    // java if (radix < Character.MIN_RADIX || radix > Character.MAX_RADIX) { throw ...; }
    if !(2..=36).contains(&radix){return None;}
    // java boolean negative = false; int i = 0, len = s.length(); int limit = -Integer.MAX_VALUE;
    let mut negative=false;let mut i=0;let len=s.len();let mut limit= -i32::MAX;
    // java if (len > 0) {
    if len>0 {
        // java char firstChar = s.charAt(0); if (firstChar < '0') {
        let firstChar=s[0];if firstChar<48 {
            // java if (firstChar == '-') { negative = true; limit = Integer.MIN_VALUE; } else if (firstChar != '+') { throw ...; }
            if firstChar==45{negative=true;limit=i32::MIN;}else if firstChar!=43{return None;}
            // java if (len == 1) { throw ...; } i++;
            if len==1{return None;}i+=1;
        }
        // java int multmin = limit / radix; int result = 0;
        let multmin=limit/radix;let mut result=0i32;
        // java while (i < len) {
        while i<len {
            // java int digit = Character.digit(s.charAt(i++), radix);
            let c=s[i];i+=1;let digit=match c{65..=90=>(c-65)as i32+10,97..=122=>(c-97)as i32+10,0xff21..=0xff3a=>(c-0xff21)as i32+10,0xff41..=0xff5a=>(c-0xff41)as i32+10,_=>characterDigit(c)};
            // java if (digit < 0 || result < multmin) { throw ...; }
            if digit<0||digit>=radix||result<multmin{return None;}
            // java result *= radix;
            result*=radix;
            // java if (result < limit + digit) { throw ...; }
            if result<limit+digit{return None;}
            // java result -= digit;
            result-=digit;
        }
        // java return negative ? result : -result;
        Some(if negative{result}else{-result})
    // java } else { throw ...; }
    }else{None}
}
// java public static long parseLong(String s, int radix) throws NumberFormatException {
pub fn parseLongRadix(s:&[u16],radix:i32)->Option<i64>{
    // java if (radix < Character.MIN_RADIX || radix > Character.MAX_RADIX) { throw ...; }
    if !(2..=36).contains(&radix){return None;}
    // java long result = 0; boolean negative = false; int i = 0, len = s.length(); long limit = -Long.MAX_VALUE;
    let mut result=0i64;let mut negative=false;let mut i=0;let len=s.len();let mut limit= -i64::MAX;
    // java if (len > 0) {
    if len>0{
        // java char firstChar = s.charAt(0); if (firstChar < '0') {
        let firstChar=s[0];if firstChar<48{
            // java if (firstChar == '-') { negative = true; limit = Long.MIN_VALUE; } else if (firstChar != '+') { throw ...; }
            if firstChar==45{negative=true;limit=i64::MIN;}else if firstChar!=43{return None;}
            // java if (len == 1) { throw ...; } i++;
            if len==1{return None;}i+=1;
        }
        // java long multmin = limit / radix;
        let multmin=limit/radix as i64;
        // java while (i < len) {
        while i<len{
            // java int digit = Character.digit(s.charAt(i++), radix);
            let c=s[i];i+=1;let digit=match c{65..=90=>(c-65)as i32+10,97..=122=>(c-97)as i32+10,0xff21..=0xff3a=>(c-0xff21)as i32+10,0xff41..=0xff5a=>(c-0xff41)as i32+10,_=>characterDigit(c)};
            // java if (digit < 0 || result < multmin) { throw ...; }
            if digit<0||digit>=radix||result<multmin{return None;}
            // java result *= radix;
            result*=radix as i64;
            // java if (result < limit + digit) { throw ...; }
            if result<limit+digit as i64{return None;}
            // java result -= digit;
            result-=digit as i64;
        }
        // java return negative ? result : -result;
        Some(if negative{result}else{-result})
    }else{
        // java throw NumberFormatException.forInputString(s, radix);
        None
    }
}
