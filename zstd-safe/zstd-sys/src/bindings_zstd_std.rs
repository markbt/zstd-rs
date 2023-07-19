/*
This file is auto-generated from the public API of the zstd library.
It is released under the same BSD license.

BSD License

For Zstandard software

Copyright (c) Meta Platforms, Inc. and affiliates. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

 * Neither the name Facebook, nor Meta, nor the names of its contributors may
   be used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* automatically generated by rust-bindgen 0.66.1 */

pub const ZSTD_VERSION_MAJOR: u32 = 1;
pub const ZSTD_VERSION_MINOR: u32 = 5;
pub const ZSTD_VERSION_RELEASE: u32 = 5;
pub const ZSTD_VERSION_NUMBER: u32 = 10505;
pub const ZSTD_CLEVEL_DEFAULT: u32 = 3;
pub const ZSTD_MAGICNUMBER: u32 = 4247762216;
pub const ZSTD_MAGIC_DICTIONARY: u32 = 3962610743;
pub const ZSTD_MAGIC_SKIPPABLE_START: u32 = 407710288;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: u32 = 4294967280;
pub const ZSTD_BLOCKSIZELOG_MAX: u32 = 17;
pub const ZSTD_BLOCKSIZE_MAX: u32 = 131072;
pub const ZSTD_CONTENTSIZE_UNKNOWN: i32 = -1;
pub const ZSTD_CONTENTSIZE_ERROR: i32 = -2;
extern "C" {
    #[doc = " ZSTD_versionNumber() :\n  Return runtime library version, the value is (MAJOR*100*100 + MINOR*100 + RELEASE)."]
    pub fn ZSTD_versionNumber() -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " ZSTD_versionString() :\n  Return runtime library version, like \"1.4.5\". Requires v1.3.0+."]
    pub fn ZSTD_versionString() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "  Simple API\n/\n/*! ZSTD_compress() :\n  Compresses `src` content as a single zstd compressed frame into already allocated `dst`.\n  NOTE: Providing `dstCapacity >= ZSTD_compressBound(srcSize)` guarantees that zstd will have\n        enough space to successfully compress the data.\n  @return : compressed size written into `dst` (<= `dstCapacity),\n            or an error code if it fails (which can be tested using ZSTD_isError())."]
    pub fn ZSTD_compress(
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        compressionLevel: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_decompress() :\n  `compressedSize` : must be the _exact_ size of some number of compressed and/or skippable frames.\n  `dstCapacity` is an upper bound of originalSize to regenerate.\n  If user cannot imply a maximum upper bound, it's better to use streaming mode to decompress data.\n  @return : the number of bytes decompressed into `dst` (<= `dstCapacity`),\n            or an errorCode if it fails (which can be tested using ZSTD_isError())."]
    pub fn ZSTD_decompress(
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        compressedSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_getFrameContentSize(
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    #[doc = " ZSTD_getDecompressedSize() :\n  NOTE: This function is now obsolete, in favor of ZSTD_getFrameContentSize().\n  Both functions work the same way, but ZSTD_getDecompressedSize() blends\n  \"empty\", \"unknown\" and \"error\" results to the same return value (0),\n  while ZSTD_getFrameContentSize() gives them separate return values.\n @return : decompressed size of `src` frame content _if known and not empty_, 0 otherwise."]
    pub fn ZSTD_getDecompressedSize(
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    #[doc = " ZSTD_findFrameCompressedSize() : Requires v1.4.0+\n `src` should point to the start of a ZSTD frame or skippable frame.\n `srcSize` must be >= first frame size\n @return : the compressed size of the first frame starting at `src`,\n           suitable to pass as `srcSize` to `ZSTD_decompress` or similar,\n        or an error code if input is invalid"]
    pub fn ZSTD_findFrameCompressedSize(
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBound(srcSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_isError(code: usize) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ZSTD_getErrorName(code: usize) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ZSTD_minCLevel() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ZSTD_maxCLevel() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ZSTD_defaultCLevel() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CCtx_s {
    _unused: [u8; 0],
}
#[doc = "  Explicit context"]
pub type ZSTD_CCtx = ZSTD_CCtx_s;
extern "C" {
    pub fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
}
extern "C" {
    pub fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> usize;
}
extern "C" {
    #[doc = " ZSTD_compressCCtx() :\n  Same as ZSTD_compress(), using an explicit ZSTD_CCtx.\n  Important : in order to behave similarly to `ZSTD_compress()`,\n  this function compresses at requested compression level,\n  __ignoring any other parameter__ .\n  If any advanced parameter was set using the advanced API,\n  they will all be reset. Only `compressionLevel` remains."]
    pub fn ZSTD_compressCCtx(
        cctx: *mut ZSTD_CCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        compressionLevel: ::std::os::raw::c_int,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DCtx_s {
    _unused: [u8; 0],
}
pub type ZSTD_DCtx = ZSTD_DCtx_s;
extern "C" {
    pub fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
}
extern "C" {
    pub fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> usize;
}
extern "C" {
    #[doc = " ZSTD_decompressDCtx() :\n  Same as ZSTD_decompress(),\n  requires an allocated ZSTD_DCtx.\n  Compatible with sticky parameters."]
    pub fn ZSTD_decompressDCtx(
        dctx: *mut ZSTD_DCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}
#[repr(u32)]
#[doc = "  Advanced compression API (Requires v1.4.0+)"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ZSTD_strategy {
    ZSTD_fast = 1,
    ZSTD_dfast = 2,
    ZSTD_greedy = 3,
    ZSTD_lazy = 4,
    ZSTD_lazy2 = 5,
    ZSTD_btlazy2 = 6,
    ZSTD_btopt = 7,
    ZSTD_btultra = 8,
    ZSTD_btultra2 = 9,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ZSTD_cParameter {
    ZSTD_c_compressionLevel = 100,
    ZSTD_c_windowLog = 101,
    ZSTD_c_hashLog = 102,
    ZSTD_c_chainLog = 103,
    ZSTD_c_searchLog = 104,
    ZSTD_c_minMatch = 105,
    ZSTD_c_targetLength = 106,
    ZSTD_c_strategy = 107,
    ZSTD_c_enableLongDistanceMatching = 160,
    ZSTD_c_ldmHashLog = 161,
    ZSTD_c_ldmMinMatch = 162,
    ZSTD_c_ldmBucketSizeLog = 163,
    ZSTD_c_ldmHashRateLog = 164,
    ZSTD_c_contentSizeFlag = 200,
    ZSTD_c_checksumFlag = 201,
    ZSTD_c_dictIDFlag = 202,
    ZSTD_c_nbWorkers = 400,
    ZSTD_c_jobSize = 401,
    ZSTD_c_overlapLog = 402,
    ZSTD_c_experimentalParam1 = 500,
    ZSTD_c_experimentalParam2 = 10,
    ZSTD_c_experimentalParam3 = 1000,
    ZSTD_c_experimentalParam4 = 1001,
    ZSTD_c_experimentalParam5 = 1002,
    ZSTD_c_experimentalParam6 = 1003,
    ZSTD_c_experimentalParam7 = 1004,
    ZSTD_c_experimentalParam8 = 1005,
    ZSTD_c_experimentalParam9 = 1006,
    ZSTD_c_experimentalParam10 = 1007,
    ZSTD_c_experimentalParam11 = 1008,
    ZSTD_c_experimentalParam12 = 1009,
    ZSTD_c_experimentalParam13 = 1010,
    ZSTD_c_experimentalParam14 = 1011,
    ZSTD_c_experimentalParam15 = 1012,
    ZSTD_c_experimentalParam16 = 1013,
    ZSTD_c_experimentalParam17 = 1014,
    ZSTD_c_experimentalParam18 = 1015,
    ZSTD_c_experimentalParam19 = 1016,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_bounds {
    pub error: usize,
    pub lowerBound: ::std::os::raw::c_int,
    pub upperBound: ::std::os::raw::c_int,
}
extern "C" {
    #[doc = " ZSTD_cParam_getBounds() :\n  All parameters must belong to an interval with lower and upper bounds,\n  otherwise they will either trigger an error or be automatically clamped.\n @return : a structure, ZSTD_bounds, which contains\n         - an error status field, which must be tested using ZSTD_isError()\n         - lower and upper bounds, both inclusive"]
    pub fn ZSTD_cParam_getBounds(cParam: ZSTD_cParameter) -> ZSTD_bounds;
}
extern "C" {
    #[doc = " ZSTD_CCtx_setParameter() :\n  Set one compression parameter, selected by enum ZSTD_cParameter.\n  All parameters have valid bounds. Bounds can be queried using ZSTD_cParam_getBounds().\n  Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).\n  Setting a parameter is generally only possible during frame initialization (before starting compression).\n  Exception : when using multi-threading mode (nbWorkers >= 1),\n              the following parameters can be updated _during_ compression (within same frame):\n              => compressionLevel, hashLog, chainLog, searchLog, minMatch, targetLength and strategy.\n              new parameters will be active for next job only (after a flush()).\n @return : an error code (which can be tested using ZSTD_isError())."]
    pub fn ZSTD_CCtx_setParameter(
        cctx: *mut ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_CCtx_setPledgedSrcSize() :\n  Total input data size to be compressed as a single frame.\n  Value will be written in frame header, unless if explicitly forbidden using ZSTD_c_contentSizeFlag.\n  This value will also be controlled at end of frame, and trigger an error if not respected.\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Note 1 : pledgedSrcSize==0 actually means zero, aka an empty frame.\n           In order to mean \"unknown content size\", pass constant ZSTD_CONTENTSIZE_UNKNOWN.\n           ZSTD_CONTENTSIZE_UNKNOWN is default value for any new frame.\n  Note 2 : pledgedSrcSize is only valid once, for the next frame.\n           It's discarded at the end of the frame, and replaced by ZSTD_CONTENTSIZE_UNKNOWN.\n  Note 3 : Whenever all input data is provided and consumed in a single round,\n           for example with ZSTD_compress2(),\n           or invoking immediately ZSTD_compressStream2(,,,ZSTD_e_end),\n           this value is automatically overridden by srcSize instead."]
    pub fn ZSTD_CCtx_setPledgedSrcSize(
        cctx: *mut ZSTD_CCtx,
        pledgedSrcSize: ::std::os::raw::c_ulonglong,
    ) -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ZSTD_ResetDirective {
    ZSTD_reset_session_only = 1,
    ZSTD_reset_parameters = 2,
    ZSTD_reset_session_and_parameters = 3,
}
extern "C" {
    #[doc = " ZSTD_CCtx_reset() :\n  There are 2 different things that can be reset, independently or jointly :\n  - The session : will stop compressing current frame, and make CCtx ready to start a new one.\n                  Useful after an error, or to interrupt any ongoing compression.\n                  Any internal data not yet flushed is cancelled.\n                  Compression parameters and dictionary remain unchanged.\n                  They will be used to compress next frame.\n                  Resetting session never fails.\n  - The parameters : changes all parameters back to \"default\".\n                  This also removes any reference to any dictionary or external sequence producer.\n                  Parameters can only be changed between 2 sessions (i.e. no compression is currently ongoing)\n                  otherwise the reset fails, and function returns an error value (which can be tested using ZSTD_isError())\n  - Both : similar to resetting the session, followed by resetting parameters."]
    pub fn ZSTD_CCtx_reset(
        cctx: *mut ZSTD_CCtx,
        reset: ZSTD_ResetDirective,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_compress2() :\n  Behave the same as ZSTD_compressCCtx(), but compression parameters are set using the advanced API.\n  ZSTD_compress2() always starts a new frame.\n  Should cctx hold data from a previously unfinished frame, everything about it is forgotten.\n  - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()\n  - The function is always blocking, returns when compression is completed.\n  NOTE: Providing `dstCapacity >= ZSTD_compressBound(srcSize)` guarantees that zstd will have\n        enough space to successfully compress the data, though it is possible it fails for other reasons.\n @return : compressed size written into `dst` (<= `dstCapacity),\n           or an error code if it fails (which can be tested using ZSTD_isError())."]
    pub fn ZSTD_compress2(
        cctx: *mut ZSTD_CCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> usize;
}
#[repr(u32)]
#[doc = "  Advanced decompression API (Requires v1.4.0+)"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ZSTD_dParameter {
    ZSTD_d_windowLogMax = 100,
    ZSTD_d_experimentalParam1 = 1000,
    ZSTD_d_experimentalParam2 = 1001,
    ZSTD_d_experimentalParam3 = 1002,
    ZSTD_d_experimentalParam4 = 1003,
    ZSTD_d_experimentalParam5 = 1004,
}
extern "C" {
    #[doc = " ZSTD_dParam_getBounds() :\n  All parameters must belong to an interval with lower and upper bounds,\n  otherwise they will either trigger an error or be automatically clamped.\n @return : a structure, ZSTD_bounds, which contains\n         - an error status field, which must be tested using ZSTD_isError()\n         - both lower and upper bounds, inclusive"]
    pub fn ZSTD_dParam_getBounds(dParam: ZSTD_dParameter) -> ZSTD_bounds;
}
extern "C" {
    #[doc = " ZSTD_DCtx_setParameter() :\n  Set one compression parameter, selected by enum ZSTD_dParameter.\n  All parameters have valid bounds. Bounds can be queried using ZSTD_dParam_getBounds().\n  Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).\n  Setting a parameter is only possible during frame initialization (before starting decompression).\n @return : 0, or an error code (which can be tested using ZSTD_isError())."]
    pub fn ZSTD_DCtx_setParameter(
        dctx: *mut ZSTD_DCtx,
        param: ZSTD_dParameter,
        value: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_DCtx_reset() :\n  Return a DCtx to clean state.\n  Session and parameters can be reset jointly or separately.\n  Parameters can only be reset when no active frame is being decompressed.\n @return : 0, or an error code, which can be tested with ZSTD_isError()"]
    pub fn ZSTD_DCtx_reset(
        dctx: *mut ZSTD_DCtx,
        reset: ZSTD_ResetDirective,
    ) -> usize;
}
#[doc = "  Streaming"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_inBuffer_s {
    #[doc = "< start of input buffer"]
    pub src: *const ::core::ffi::c_void,
    #[doc = "< size of input buffer"]
    pub size: usize,
    #[doc = "< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size"]
    pub pos: usize,
}
#[doc = "  Streaming"]
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_outBuffer_s {
    #[doc = "< start of output buffer"]
    pub dst: *mut ::core::ffi::c_void,
    #[doc = "< size of output buffer"]
    pub size: usize,
    #[doc = "< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size"]
    pub pos: usize,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_CStream = ZSTD_CCtx;
extern "C" {
    pub fn ZSTD_createCStream() -> *mut ZSTD_CStream;
}
extern "C" {
    pub fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ZSTD_EndDirective {
    ZSTD_e_continue = 0,
    ZSTD_e_flush = 1,
    ZSTD_e_end = 2,
}
extern "C" {
    #[doc = " ZSTD_compressStream2() : Requires v1.4.0+\n  Behaves about the same as ZSTD_compressStream, with additional control on end directive.\n  - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()\n  - Compression parameters cannot be changed once compression is started (save a list of exceptions in multi-threading mode)\n  - output->pos must be <= dstCapacity, input->pos must be <= srcSize\n  - output->pos and input->pos will be updated. They are guaranteed to remain below their respective limit.\n  - endOp must be a valid directive\n  - When nbWorkers==0 (default), function is blocking : it completes its job before returning to caller.\n  - When nbWorkers>=1, function is non-blocking : it copies a portion of input, distributes jobs to internal worker threads, flush to output whatever is available,\n                                                  and then immediately returns, just indicating that there is some data remaining to be flushed.\n                                                  The function nonetheless guarantees forward progress : it will return only after it reads or write at least 1+ byte.\n  - Exception : if the first call requests a ZSTD_e_end directive and provides enough dstCapacity, the function delegates to ZSTD_compress2() which is always blocking.\n  - @return provides a minimum amount of data remaining to be flushed from internal buffers\n            or an error code, which can be tested using ZSTD_isError().\n            if @return != 0, flush is not fully completed, there is still some data left within internal buffers.\n            This is useful for ZSTD_e_flush, since in this case more flushes are necessary to empty all buffers.\n            For ZSTD_e_end, @return == 0 when internal buffers are fully flushed and frame is completed.\n  - after a ZSTD_e_end directive, if internal buffer is not fully flushed (@return != 0),\n            only ZSTD_e_end or ZSTD_e_flush operations are allowed.\n            Before starting a new compression job, or changing compression parameters,\n            it is required to fully flush internal buffers."]
    pub fn ZSTD_compressStream2(
        cctx: *mut ZSTD_CCtx,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
        endOp: ZSTD_EndDirective,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_CStreamInSize() -> usize;
}
extern "C" {
    pub fn ZSTD_CStreamOutSize() -> usize;
}
extern "C" {
    #[doc = " Equivalent to:\n\n     ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);\n     ZSTD_CCtx_refCDict(zcs, NULL); // clear the dictionary (if any)\n     ZSTD_CCtx_setParameter(zcs, ZSTD_c_compressionLevel, compressionLevel);\n\n Note that ZSTD_initCStream() clears any previously set dictionary. Use the new API\n to compress with a dictionary."]
    pub fn ZSTD_initCStream(
        zcs: *mut ZSTD_CStream,
        compressionLevel: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    #[doc = " Alternative for ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue).\n NOTE: The return value is different. ZSTD_compressStream() returns a hint for\n the next read size (if non-zero and not an error). ZSTD_compressStream2()\n returns the minimum nb of bytes left to flush (if non-zero and not an error)."]
    pub fn ZSTD_compressStream(
        zcs: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> usize;
}
extern "C" {
    #[doc = " Equivalent to ZSTD_compressStream2(zcs, output, &emptyInput, ZSTD_e_flush)."]
    pub fn ZSTD_flushStream(
        zcs: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
    ) -> usize;
}
extern "C" {
    #[doc = " Equivalent to ZSTD_compressStream2(zcs, output, &emptyInput, ZSTD_e_end)."]
    pub fn ZSTD_endStream(
        zcs: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
    ) -> usize;
}
pub type ZSTD_DStream = ZSTD_DCtx;
extern "C" {
    pub fn ZSTD_createDStream() -> *mut ZSTD_DStream;
}
extern "C" {
    pub fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> usize;
}
extern "C" {
    #[doc = " ZSTD_initDStream() :\n Initialize/reset DStream state for new decompression operation.\n Call before new decompression operation using same DStream.\n\n Note : This function is redundant with the advanced API and equivalent to:\n     ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);\n     ZSTD_DCtx_refDDict(zds, NULL);"]
    pub fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> usize;
}
extern "C" {
    #[doc = " ZSTD_decompressStream() :\n Streaming decompression function.\n Call repetitively to consume full input updating it as necessary.\n Function will update both input and output `pos` fields exposing current state via these fields:\n - `input.pos < input.size`, some input remaining and caller should provide remaining input\n   on the next call.\n - `output.pos < output.size`, decoder finished and flushed all remaining buffers.\n - `output.pos == output.size`, potentially uncflushed data present in the internal buffers,\n   call ZSTD_decompressStream() again to flush remaining data to output.\n Note : with no additional input, amount of data flushed <= ZSTD_BLOCKSIZE_MAX.\n\n @return : 0 when a frame is completely decoded and fully flushed,\n           or an error code, which can be tested using ZSTD_isError(),\n           or any other value > 0, which means there is some decoding or flushing to do to complete current frame."]
    pub fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> usize;
}
extern "C" {
    pub fn ZSTD_DStreamInSize() -> usize;
}
extern "C" {
    pub fn ZSTD_DStreamOutSize() -> usize;
}
extern "C" {
    #[doc = "  Simple dictionary API\n/\n/*! ZSTD_compress_usingDict() :\n  Compression at an explicit compression level using a Dictionary.\n  A dictionary can be any arbitrary data segment (also called a prefix),\n  or a buffer with specified information (see zdict.h).\n  Note : This function loads the dictionary, resulting in significant startup delay.\n         It's intended for a dictionary used only once.\n  Note 2 : When `dict == NULL || dictSize < 8` no dictionary is used."]
    pub fn ZSTD_compress_usingDict(
        ctx: *mut ZSTD_CCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        dict: *const ::core::ffi::c_void,
        dictSize: usize,
        compressionLevel: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_decompress_usingDict() :\n  Decompression using a known Dictionary.\n  Dictionary must be identical to the one used during compression.\n  Note : This function loads the dictionary, resulting in significant startup delay.\n         It's intended for a dictionary used only once.\n  Note : When `dict == NULL || dictSize < 8` no dictionary is used."]
    pub fn ZSTD_decompress_usingDict(
        dctx: *mut ZSTD_DCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        dict: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CDict_s {
    _unused: [u8; 0],
}
#[doc = "  Bulk processing dictionary API"]
pub type ZSTD_CDict = ZSTD_CDict_s;
extern "C" {
    #[doc = " ZSTD_createCDict() :\n  When compressing multiple messages or blocks using the same dictionary,\n  it's recommended to digest the dictionary only once, since it's a costly operation.\n  ZSTD_createCDict() will create a state from digesting a dictionary.\n  The resulting state can be used for future compression operations with very limited startup cost.\n  ZSTD_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.\n @dictBuffer can be released after ZSTD_CDict creation, because its content is copied within CDict.\n  Note 1 : Consider experimental function `ZSTD_createCDict_byReference()` if you prefer to not duplicate @dictBuffer content.\n  Note 2 : A ZSTD_CDict can be created from an empty @dictBuffer,\n      in which case the only thing that it transports is the @compressionLevel.\n      This can be useful in a pipeline featuring ZSTD_compress_usingCDict() exclusively,\n      expecting a ZSTD_CDict parameter with any data, including those without a known dictionary."]
    pub fn ZSTD_createCDict(
        dictBuffer: *const ::core::ffi::c_void,
        dictSize: usize,
        compressionLevel: ::std::os::raw::c_int,
    ) -> *mut ZSTD_CDict;
}
extern "C" {
    #[doc = " ZSTD_freeCDict() :\n  Function frees memory allocated by ZSTD_createCDict().\n  If a NULL pointer is passed, no operation is performed."]
    pub fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> usize;
}
extern "C" {
    #[doc = " ZSTD_compress_usingCDict() :\n  Compression using a digested Dictionary.\n  Recommended when same dictionary is used multiple times.\n  Note : compression level is _decided at dictionary creation time_,\n     and frame parameters are hardcoded (dictID=yes, contentSize=yes, checksum=no)"]
    pub fn ZSTD_compress_usingCDict(
        cctx: *mut ZSTD_CCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        cdict: *const ZSTD_CDict,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DDict_s {
    _unused: [u8; 0],
}
pub type ZSTD_DDict = ZSTD_DDict_s;
extern "C" {
    #[doc = " ZSTD_createDDict() :\n  Create a digested dictionary, ready to start decompression operation without startup delay.\n  dictBuffer can be released after DDict creation, as its content is copied inside DDict."]
    pub fn ZSTD_createDDict(
        dictBuffer: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> *mut ZSTD_DDict;
}
extern "C" {
    #[doc = " ZSTD_freeDDict() :\n  Function frees memory allocated with ZSTD_createDDict()\n  If a NULL pointer is passed, no operation is performed."]
    pub fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> usize;
}
extern "C" {
    #[doc = " ZSTD_decompress_usingDDict() :\n  Decompression using a digested Dictionary.\n  Recommended when same dictionary is used multiple times."]
    pub fn ZSTD_decompress_usingDDict(
        dctx: *mut ZSTD_DCtx,
        dst: *mut ::core::ffi::c_void,
        dstCapacity: usize,
        src: *const ::core::ffi::c_void,
        srcSize: usize,
        ddict: *const ZSTD_DDict,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_getDictID_fromDict() : Requires v1.4.0+\n  Provides the dictID stored within dictionary.\n  if @return == 0, the dictionary is not conformant with Zstandard specification.\n  It can still be loaded, but as a content-only dictionary."]
    pub fn ZSTD_getDictID_fromDict(
        dict: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " ZSTD_getDictID_fromCDict() : Requires v1.5.0+\n  Provides the dictID of the dictionary loaded into `cdict`.\n  If @return == 0, the dictionary is not conformant to Zstandard specification, or empty.\n  Non-conformant dictionaries can still be loaded, but as content-only dictionaries."]
    pub fn ZSTD_getDictID_fromCDict(
        cdict: *const ZSTD_CDict,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " ZSTD_getDictID_fromDDict() : Requires v1.4.0+\n  Provides the dictID of the dictionary loaded into `ddict`.\n  If @return == 0, the dictionary is not conformant to Zstandard specification, or empty.\n  Non-conformant dictionaries can still be loaded, but as content-only dictionaries."]
    pub fn ZSTD_getDictID_fromDDict(
        ddict: *const ZSTD_DDict,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " ZSTD_getDictID_fromFrame() : Requires v1.4.0+\n  Provides the dictID required to decompressed the frame stored within `src`.\n  If @return == 0, the dictID could not be decoded.\n  This could for one of the following reasons :\n  - The frame does not require a dictionary to be decoded (most common case).\n  - The frame was built with dictID intentionally removed. Whatever dictionary is necessary is a hidden piece of information.\n    Note : this use case also happens when using a non-conformant dictionary.\n  - `srcSize` is too small, and as a result, the frame header could not be decoded (only possible if `srcSize < ZSTD_FRAMEHEADERSIZE_MAX`).\n  - This is not a Zstandard frame.\n  When identifying the exact failure cause, it's possible to use ZSTD_getFrameHeader(), which will provide a more precise error code."]
    pub fn ZSTD_getDictID_fromFrame(
        src: *const ::core::ffi::c_void,
        srcSize: usize,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " ZSTD_CCtx_loadDictionary() : Requires v1.4.0+\n  Create an internal CDict from `dict` buffer.\n  Decompression will have to use same dictionary.\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Special: Loading a NULL (or 0-size) dictionary invalidates previous dictionary,\n           meaning \"return to no-dictionary mode\".\n  Note 1 : Dictionary is sticky, it will be used for all future compressed frames,\n           until parameters are reset, a new dictionary is loaded, or the dictionary\n           is explicitly invalidated by loading a NULL dictionary.\n  Note 2 : Loading a dictionary involves building tables.\n           It's also a CPU consuming operation, with non-negligible impact on latency.\n           Tables are dependent on compression parameters, and for this reason,\n           compression parameters can no longer be changed after loading a dictionary.\n  Note 3 :`dict` content will be copied internally.\n           Use experimental ZSTD_CCtx_loadDictionary_byReference() to reference content instead.\n           In such a case, dictionary buffer must outlive its users.\n  Note 4 : Use ZSTD_CCtx_loadDictionary_advanced()\n           to precisely select how dictionary content must be interpreted.\n  Note 5 : This method does not benefit from LDM (long distance mode).\n           If you want to employ LDM on some large dictionary content,\n           prefer employing ZSTD_CCtx_refPrefix() described below."]
    pub fn ZSTD_CCtx_loadDictionary(
        cctx: *mut ZSTD_CCtx,
        dict: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_CCtx_refCDict() : Requires v1.4.0+\n  Reference a prepared dictionary, to be used for all future compressed frames.\n  Note that compression parameters are enforced from within CDict,\n  and supersede any compression parameter previously set within CCtx.\n  The parameters ignored are labelled as \"superseded-by-cdict\" in the ZSTD_cParameter enum docs.\n  The ignored parameters will be used again if the CCtx is returned to no-dictionary mode.\n  The dictionary will remain valid for future compressed frames using same CCtx.\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Special : Referencing a NULL CDict means \"return to no-dictionary mode\".\n  Note 1 : Currently, only one dictionary can be managed.\n           Referencing a new dictionary effectively \"discards\" any previous one.\n  Note 2 : CDict is just referenced, its lifetime must outlive its usage within CCtx."]
    pub fn ZSTD_CCtx_refCDict(
        cctx: *mut ZSTD_CCtx,
        cdict: *const ZSTD_CDict,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_CCtx_refPrefix() : Requires v1.4.0+\n  Reference a prefix (single-usage dictionary) for next compressed frame.\n  A prefix is **only used once**. Tables are discarded at end of frame (ZSTD_e_end).\n  Decompression will need same prefix to properly regenerate data.\n  Compressing with a prefix is similar in outcome as performing a diff and compressing it,\n  but performs much faster, especially during decompression (compression speed is tunable with compression level).\n  This method is compatible with LDM (long distance mode).\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Special: Adding any prefix (including NULL) invalidates any previous prefix or dictionary\n  Note 1 : Prefix buffer is referenced. It **must** outlive compression.\n           Its content must remain unmodified during compression.\n  Note 2 : If the intention is to diff some large src data blob with some prior version of itself,\n           ensure that the window size is large enough to contain the entire source.\n           See ZSTD_c_windowLog.\n  Note 3 : Referencing a prefix involves building tables, which are dependent on compression parameters.\n           It's a CPU consuming operation, with non-negligible impact on latency.\n           If there is a need to use the same prefix multiple times, consider loadDictionary instead.\n  Note 4 : By default, the prefix is interpreted as raw content (ZSTD_dct_rawContent).\n           Use experimental ZSTD_CCtx_refPrefix_advanced() to alter dictionary interpretation."]
    pub fn ZSTD_CCtx_refPrefix(
        cctx: *mut ZSTD_CCtx,
        prefix: *const ::core::ffi::c_void,
        prefixSize: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_DCtx_loadDictionary() : Requires v1.4.0+\n  Create an internal DDict from dict buffer, to be used to decompress all future frames.\n  The dictionary remains valid for all future frames, until explicitly invalidated, or\n  a new dictionary is loaded.\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Special : Adding a NULL (or 0-size) dictionary invalidates any previous dictionary,\n            meaning \"return to no-dictionary mode\".\n  Note 1 : Loading a dictionary involves building tables,\n           which has a non-negligible impact on CPU usage and latency.\n           It's recommended to \"load once, use many times\", to amortize the cost\n  Note 2 :`dict` content will be copied internally, so `dict` can be released after loading.\n           Use ZSTD_DCtx_loadDictionary_byReference() to reference dictionary content instead.\n  Note 3 : Use ZSTD_DCtx_loadDictionary_advanced() to take control of\n           how dictionary content is loaded and interpreted."]
    pub fn ZSTD_DCtx_loadDictionary(
        dctx: *mut ZSTD_DCtx,
        dict: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_DCtx_refDDict() : Requires v1.4.0+\n  Reference a prepared dictionary, to be used to decompress next frames.\n  The dictionary remains active for decompression of future frames using same DCtx.\n\n  If called with ZSTD_d_refMultipleDDicts enabled, repeated calls of this function\n  will store the DDict references in a table, and the DDict used for decompression\n  will be determined at decompression time, as per the dict ID in the frame.\n  The memory for the table is allocated on the first call to refDDict, and can be\n  freed with ZSTD_freeDCtx().\n\n  If called with ZSTD_d_refMultipleDDicts disabled (the default), only one dictionary\n  will be managed, and referencing a dictionary effectively \"discards\" any previous one.\n\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Special: referencing a NULL DDict means \"return to no-dictionary mode\".\n  Note 2 : DDict is just referenced, its lifetime must outlive its usage from DCtx."]
    pub fn ZSTD_DCtx_refDDict(
        dctx: *mut ZSTD_DCtx,
        ddict: *const ZSTD_DDict,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_DCtx_refPrefix() : Requires v1.4.0+\n  Reference a prefix (single-usage dictionary) to decompress next frame.\n  This is the reverse operation of ZSTD_CCtx_refPrefix(),\n  and must use the same prefix as the one used during compression.\n  Prefix is **only used once**. Reference is discarded at end of frame.\n  End of frame is reached when ZSTD_decompressStream() returns 0.\n @result : 0, or an error code (which can be tested with ZSTD_isError()).\n  Note 1 : Adding any prefix (including NULL) invalidates any previously set prefix or dictionary\n  Note 2 : Prefix buffer is referenced. It **must** outlive decompression.\n           Prefix buffer must remain unmodified up to the end of frame,\n           reached when ZSTD_decompressStream() returns 0.\n  Note 3 : By default, the prefix is treated as raw content (ZSTD_dct_rawContent).\n           Use ZSTD_CCtx_refPrefix_advanced() to alter dictMode (Experimental section)\n  Note 4 : Referencing a raw content prefix has almost no cpu nor memory cost.\n           A full dictionary is more costly, as it requires building tables."]
    pub fn ZSTD_DCtx_refPrefix(
        dctx: *mut ZSTD_DCtx,
        prefix: *const ::core::ffi::c_void,
        prefixSize: usize,
    ) -> usize;
}
extern "C" {
    #[doc = " ZSTD_sizeof_*() : Requires v1.4.0+\n  These functions give the _current_ memory usage of selected object.\n  Note that object memory usage can evolve (increase or decrease) over time."]
    pub fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_DCtx(dctx: *const ZSTD_DCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_CStream(zcs: *const ZSTD_CStream) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_DStream(zds: *const ZSTD_DStream) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> usize;
}
