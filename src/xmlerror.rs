//!  xmlerror
//!
//! xmlerror - error handling
//!
//! the API used to report errors
//!
//! Author(s): Daniel Veillard

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::*;
use std::os::raw::{c_int as int, c_void as void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlError {
    // What part of the library raised this error
    domain: int,
    //  The error code, e.g. an xmlParserError
    code: int,
    //  human-readable informative error message
    message: *mut c_char,
    //  how consequent is the error
    level: xmlErrorLevel,
    //  the filename
    file: *mut c_char,
    //  the line number if available
    line: int,
    //  extra string information
    str1: *mut c_char,
    //  extra string information
    str2: *mut c_char,
    //  extra string information
    str3: *mut c_char,
    //  extra number information
    int1: int,
    //  error column # or 0 if N/A (todo: rename field when we would brk ABI)
    int2: int,
    //  the parser context if available
    ctxt: *mut void,
    //  the node in the tree
    node: *mut void,
}

pub type xmlErrorDomain = u32;

pub const xmlErrorDomain_XML_FROM_NONE: xmlErrorDomain = 0;
/* The XML parser */
pub const xmlErrorDomain_XML_FROM_PARSER: xmlErrorDomain = 1;
/* The tree module */
pub const xmlErrorDomain_XML_FROM_TREE: xmlErrorDomain = 2;
/* The XML Namespace module */
pub const xmlErrorDomain_XML_FROM_NAMESPACE: xmlErrorDomain = 3;
/* The XML DTD validation with parser contex */
pub const xmlErrorDomain_XML_FROM_DTD: xmlErrorDomain = 4;
/* The HTML parser */
pub const xmlErrorDomain_XML_FROM_HTML: xmlErrorDomain = 5;
/* The memory allocator */
pub const xmlErrorDomain_XML_FROM_MEMORY: xmlErrorDomain = 6;
/* The serialization code */
pub const xmlErrorDomain_XML_FROM_OUTPUT: xmlErrorDomain = 7;
/* The Input/Output stack */
pub const xmlErrorDomain_XML_FROM_IO: xmlErrorDomain = 8;
/* The FTP module */
pub const xmlErrorDomain_XML_FROM_FTP: xmlErrorDomain = 9;
/* The HTTP module */
pub const xmlErrorDomain_XML_FROM_HTTP: xmlErrorDomain = 10;
/* The XInclude processing */
pub const xmlErrorDomain_XML_FROM_XINCLUDE: xmlErrorDomain = 11;
/* The XPath module */
pub const xmlErrorDomain_XML_FROM_XPATH: xmlErrorDomain = 12;
/* The XPointer module */
pub const xmlErrorDomain_XML_FROM_XPOINTER: xmlErrorDomain = 13;
/* The regular expressions module */
pub const xmlErrorDomain_XML_FROM_REGEXP: xmlErrorDomain = 14;
/* The W3C XML Schemas Datatype module */
pub const xmlErrorDomain_XML_FROM_DATATYPE: xmlErrorDomain = 15;
/* The W3C XML Schemas parser module */
pub const xmlErrorDomain_FROM_SCHEMASP: xmlErrorDomain = 16;
/* The W3C XML Schemas validation module */
pub const xmlErrorDomain_XML_FROM_SCHEMASV: xmlErrorDomain = 17;
/* The Relax-NG parser module */
pub const xmlErrorDomain_XML_FROM_RELAXNGP: xmlErrorDomain = 18;
/* The Relax-NG validator module */
pub const xmlErrorDomain_XML_FROM_RELAXNGV: xmlErrorDomain = 19;
/* The Catalog module */
pub const xmlErrorDomain_XML_FROM_CATALOG: xmlErrorDomain = 20;
/* The Canonicalization module */
pub const xmlErrorDomain_XML_FROM_C14N: xmlErrorDomain = 21;
/* The XSLT engine from libxslt */
pub const xmlErrorDomain_XML_FROM_XSLT: xmlErrorDomain = 22;
/* The XML DTD validation with valid context */
pub const xmlErrorDomain_XML_FROM_VALID: xmlErrorDomain = 23;
/* The error checking module */
pub const xmlErrorDomain_XML_FROM_CHECK: xmlErrorDomain = 24;
/* The xmlwriter module */
pub const xmlErrorDomain_XML_FROM_WRITER: xmlErrorDomain = 25;
/* The dynamically loaded module modul */
pub const xmlErrorDomain_XML_FROM_MODULE: xmlErrorDomain = 26;
/* The module handling character conversion */
pub const xmlErrorDomain_XML_FROM_I18N: xmlErrorDomain = 27;
/* The Schematron validator module */
pub const xmlErrorDomain_XML_FROM_SCHEMATRONV: xmlErrorDomain = 28;
/* The buffers module */
pub const xmlErrorDomain_XML_FROM_BUFFER: xmlErrorDomain = 29;
/*  The URI module */
pub const xmlErrorDomain_XML_FROM_URI: xmlErrorDomain = 30;

pub type xmlErrorLevel = u32;

pub const xmlErrorLevel_XML_ERR_NONE: xmlErrorLevel = 0;
/* A simple warning */
pub const xmlErrorLevel_XML_ERR_WARNING: xmlErrorLevel = 1;
/* A recoverable error */
pub const xmlErrorLevel_XML_ERR_ERROR: xmlErrorLevel = 2;
/*  A fatal error */
pub const xmlErrorLevel_XML_ERR_FATAL: xmlErrorLevel = 3;

pub type xmlError = _xmlError;
pub type xmlErrorPtr = *mut xmlError;

pub type xmlParserErrors = u32;

pub const xmlParserErrors_XML_ERR_OK: xmlParserErrors = 0;
pub const xmlParserErrors_XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const xmlParserErrors_XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const xmlParserErrors_XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const xmlParserErrors_XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const xmlParserErrors_XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const xmlParserErrors_XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const xmlParserErrors_XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const xmlParserErrors_XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const xmlParserErrors_XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const xmlParserErrors_XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const xmlParserErrors_XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const xmlParserErrors_XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const xmlParserErrors_XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const xmlParserErrors_XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const xmlParserErrors_XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const xmlParserErrors_XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const xmlParserErrors_XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const xmlParserErrors_XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const xmlParserErrors_XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const xmlParserErrors_XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const xmlParserErrors_XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const xmlParserErrors_XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const xmlParserErrors_XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const xmlParserErrors_XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const xmlParserErrors_XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const xmlParserErrors_XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const xmlParserErrors_XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const xmlParserErrors_XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const xmlParserErrors_XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const xmlParserErrors_XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const xmlParserErrors_XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const xmlParserErrors_XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const xmlParserErrors_XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const xmlParserErrors_XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const xmlParserErrors_XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const xmlParserErrors_XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const xmlParserErrors_XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const xmlParserErrors_XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const xmlParserErrors_XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const xmlParserErrors_XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const xmlParserErrors_XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const xmlParserErrors_XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const xmlParserErrors_XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const xmlParserErrors_XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const xmlParserErrors_XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const xmlParserErrors_XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const xmlParserErrors_XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const xmlParserErrors_XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const xmlParserErrors_XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const xmlParserErrors_XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const xmlParserErrors_XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const xmlParserErrors_XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const xmlParserErrors_XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const xmlParserErrors_XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const xmlParserErrors_XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const xmlParserErrors_XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const xmlParserErrors_XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const xmlParserErrors_XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const xmlParserErrors_XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const xmlParserErrors_XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const xmlParserErrors_XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const xmlParserErrors_XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const xmlParserErrors_XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const xmlParserErrors_XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const xmlParserErrors_XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const xmlParserErrors_XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const xmlParserErrors_XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const xmlParserErrors_XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const xmlParserErrors_XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const xmlParserErrors_XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const xmlParserErrors_XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const xmlParserErrors_XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const xmlParserErrors_XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const xmlParserErrors_XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const xmlParserErrors_XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const xmlParserErrors_XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const xmlParserErrors_XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const xmlParserErrors_XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const xmlParserErrors_XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const xmlParserErrors_XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const xmlParserErrors_XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const xmlParserErrors_XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const xmlParserErrors_XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const xmlParserErrors_XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const xmlParserErrors_XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const xmlParserErrors_XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const xmlParserErrors_XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const xmlParserErrors_XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const xmlParserErrors_XML_WAR_NS_URI: xmlParserErrors = 99;
pub const xmlParserErrors_XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const xmlParserErrors_XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const xmlParserErrors_XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const xmlParserErrors_XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const xmlParserErrors_XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const xmlParserErrors_XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const xmlParserErrors_XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const xmlParserErrors_XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const xmlParserErrors_XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const xmlParserErrors_XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const xmlParserErrors_XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const xmlParserErrors_XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const xmlParserErrors_XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const xmlParserErrors_XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const xmlParserErrors_XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const xmlParserErrors_XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const xmlParserErrors_XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const xmlParserErrors_XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const xmlParserErrors_XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const xmlParserErrors_XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const xmlParserErrors_XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const xmlParserErrors_XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const xmlParserErrors_XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const xmlParserErrors_XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const xmlParserErrors_XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const xmlParserErrors_XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const xmlParserErrors_XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const xmlParserErrors_XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const xmlParserErrors_XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const xmlParserErrors_XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const xmlParserErrors_XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const xmlParserErrors_XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const xmlParserErrors_XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const xmlParserErrors_XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const xmlParserErrors_XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const xmlParserErrors_XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const xmlParserErrors_XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const xmlParserErrors_XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const xmlParserErrors_XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const xmlParserErrors_XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const xmlParserErrors_XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const xmlParserErrors_XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const xmlParserErrors_XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const xmlParserErrors_XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const xmlParserErrors_XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const xmlParserErrors_XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const xmlParserErrors_XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const xmlParserErrors_XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const xmlParserErrors_XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const xmlParserErrors_XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const xmlParserErrors_XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const xmlParserErrors_XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const xmlParserErrors_XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const xmlParserErrors_XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const xmlParserErrors_XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const xmlParserErrors_XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1_000;
pub const xmlParserErrors_XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1_001;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1_002;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1_003;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1_004;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1_005;
pub const xmlParserErrors_XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1_006;
pub const xmlParserErrors_XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1_007;
pub const xmlParserErrors_XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1_008;
pub const xmlParserErrors_XML_RNGP_DATA_CONTENT: xmlParserErrors = 1_009;
pub const xmlParserErrors_XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1_010;
pub const xmlParserErrors_XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1_011;
pub const xmlParserErrors_XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1_012;
pub const xmlParserErrors_XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1_013;
pub const xmlParserErrors_XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1_014;
pub const xmlParserErrors_XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1_015;
pub const xmlParserErrors_XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1_016;
pub const xmlParserErrors_XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1_017;
pub const xmlParserErrors_XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1_018;
pub const xmlParserErrors_XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1_019;
pub const xmlParserErrors_XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1_020;
pub const xmlParserErrors_XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1_021;
pub const xmlParserErrors_XML_RNGP_EMPTY: xmlParserErrors = 1_022;
pub const xmlParserErrors_XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1_023;
pub const xmlParserErrors_XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1_024;
pub const xmlParserErrors_XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1_025;
pub const xmlParserErrors_XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1_026;
pub const xmlParserErrors_XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1_027;
pub const xmlParserErrors_XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1_028;
pub const xmlParserErrors_XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1_029;
pub const xmlParserErrors_XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1_030;
pub const xmlParserErrors_XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1_031;
pub const xmlParserErrors_XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1_032;
pub const xmlParserErrors_XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1_033;
pub const xmlParserErrors_XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1_034;
pub const xmlParserErrors_XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1_035;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1_036;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1_037;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1_038;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1_039;
pub const xmlParserErrors_XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1_040;
pub const xmlParserErrors_XML_RNGP_HREF_ERROR: xmlParserErrors = 1_041;
pub const xmlParserErrors_XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1_042;
pub const xmlParserErrors_XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1_043;
pub const xmlParserErrors_XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1_044;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1_045;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1_046;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1_047;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1_048;
pub const xmlParserErrors_XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1_049;
pub const xmlParserErrors_XML_RNGP_INVALID_URI: xmlParserErrors = 1_050;
pub const xmlParserErrors_XML_RNGP_INVALID_VALUE: xmlParserErrors = 1_051;
pub const xmlParserErrors_XML_RNGP_MISSING_HREF: xmlParserErrors = 1_052;
pub const xmlParserErrors_XML_RNGP_NAME_MISSING: xmlParserErrors = 1_053;
pub const xmlParserErrors_XML_RNGP_NEED_COMBINE: xmlParserErrors = 1_054;
pub const xmlParserErrors_XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1_055;
pub const xmlParserErrors_XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1_056;
pub const xmlParserErrors_XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1_057;
pub const xmlParserErrors_XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1_058;
pub const xmlParserErrors_XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1_059;
pub const xmlParserErrors_XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1_060;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1_061;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1_062;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1_063;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1_064;
pub const xmlParserErrors_XML_RNGP_PARSE_ERROR: xmlParserErrors = 1_065;
pub const xmlParserErrors_XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1_066;
pub const xmlParserErrors_XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1_067;
pub const xmlParserErrors_XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1_068;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1_069;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1_070;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1_071;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1_072;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1_073;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1_074;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1_075;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1_076;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1_077;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1_078;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1_079;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1_080;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1_081;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1_082;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1_083;
pub const xmlParserErrors_XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1_084;
pub const xmlParserErrors_XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1_085;
pub const xmlParserErrors_XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1_086;
pub const xmlParserErrors_XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1_087;
pub const xmlParserErrors_XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1_088;
pub const xmlParserErrors_XML_RNGP_PAT_START_DATA: xmlParserErrors = 1_089;
pub const xmlParserErrors_XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1_090;
pub const xmlParserErrors_XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1_091;
pub const xmlParserErrors_XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1_092;
pub const xmlParserErrors_XML_RNGP_PAT_START_LIST: xmlParserErrors = 1_093;
pub const xmlParserErrors_XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1_094;
pub const xmlParserErrors_XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1_095;
pub const xmlParserErrors_XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1_096;
pub const xmlParserErrors_XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1_097;
pub const xmlParserErrors_XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1_098;
pub const xmlParserErrors_XML_RNGP_REF_CYCLE: xmlParserErrors = 1_099;
pub const xmlParserErrors_XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1_100;
pub const xmlParserErrors_XML_RNGP_REF_NO_DEF: xmlParserErrors = 1_101;
pub const xmlParserErrors_XML_RNGP_REF_NO_NAME: xmlParserErrors = 1_102;
pub const xmlParserErrors_XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1_103;
pub const xmlParserErrors_XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1_104;
pub const xmlParserErrors_XML_RNGP_START_CONTENT: xmlParserErrors = 1_105;
pub const xmlParserErrors_XML_RNGP_START_EMPTY: xmlParserErrors = 1_106;
pub const xmlParserErrors_XML_RNGP_START_MISSING: xmlParserErrors = 1_107;
pub const xmlParserErrors_XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1_108;
pub const xmlParserErrors_XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1_109;
pub const xmlParserErrors_XML_RNGP_TYPE_MISSING: xmlParserErrors = 1_110;
pub const xmlParserErrors_XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1_111;
pub const xmlParserErrors_XML_RNGP_TYPE_VALUE: xmlParserErrors = 1_112;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1_113;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1_114;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1_115;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1_116;
pub const xmlParserErrors_XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1_117;
pub const xmlParserErrors_XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1_118;
pub const xmlParserErrors_XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1_119;
pub const xmlParserErrors_XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1_120;
pub const xmlParserErrors_XML_RNGP_XMLNS_NAME: xmlParserErrors = 1_121;
pub const xmlParserErrors_XML_RNGP_XML_NS: xmlParserErrors = 1_122;
pub const xmlParserErrors_XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1_200;
pub const xmlParserErrors_XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1_201;
pub const xmlParserErrors_XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1_202;
pub const xmlParserErrors_XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1_203;
pub const xmlParserErrors_XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1_204;
pub const xmlParserErrors_XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1_205;
pub const xmlParserErrors_XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1_206;
pub const xmlParserErrors_XML_XPATH_EXPR_ERROR: xmlParserErrors = 1_207;
pub const xmlParserErrors_XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1_208;
pub const xmlParserErrors_XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1_209;
pub const xmlParserErrors_XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1_210;
pub const xmlParserErrors_XML_XPATH_INVALID_TYPE: xmlParserErrors = 1_211;
pub const xmlParserErrors_XML_XPATH_INVALID_ARITY: xmlParserErrors = 1_212;
pub const xmlParserErrors_XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1_213;
pub const xmlParserErrors_XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1_214;
pub const xmlParserErrors_XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1_215;
pub const xmlParserErrors_XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1_216;
pub const xmlParserErrors_XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1_217;
pub const xmlParserErrors_XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1_218;
pub const xmlParserErrors_XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1_219;
pub const xmlParserErrors_XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1_220;
pub const xmlParserErrors_XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1_221;
pub const xmlParserErrors_XML_TREE_INVALID_HEX: xmlParserErrors = 1_300;
pub const xmlParserErrors_XML_TREE_INVALID_DEC: xmlParserErrors = 1_301;
pub const xmlParserErrors_XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1_302;
pub const xmlParserErrors_XML_TREE_NOT_UTF8: xmlParserErrors = 1_303;
pub const xmlParserErrors_XML_SAVE_NOT_UTF8: xmlParserErrors = 1_400;
pub const xmlParserErrors_XML_SAVE_CHAR_INVALID: xmlParserErrors = 1_401;
pub const xmlParserErrors_XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1_402;
pub const xmlParserErrors_XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1_403;
pub const xmlParserErrors_XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1_450;
pub const xmlParserErrors_XML_IO_UNKNOWN: xmlParserErrors = 1_500;
pub const xmlParserErrors_XML_IO_EACCES: xmlParserErrors = 1_501;
pub const xmlParserErrors_XML_IO_EAGAIN: xmlParserErrors = 1_502;
pub const xmlParserErrors_XML_IO_EBADF: xmlParserErrors = 1_503;
pub const xmlParserErrors_XML_IO_EBADMSG: xmlParserErrors = 1_504;
pub const xmlParserErrors_XML_IO_EBUSY: xmlParserErrors = 1_505;
pub const xmlParserErrors_XML_IO_ECANCELED: xmlParserErrors = 1_506;
pub const xmlParserErrors_XML_IO_ECHILD: xmlParserErrors = 1_507;
pub const xmlParserErrors_XML_IO_EDEADLK: xmlParserErrors = 1_508;
pub const xmlParserErrors_XML_IO_EDOM: xmlParserErrors = 1_509;
pub const xmlParserErrors_XML_IO_EEXIST: xmlParserErrors = 1_510;
pub const xmlParserErrors_XML_IO_EFAULT: xmlParserErrors = 1_511;
pub const xmlParserErrors_XML_IO_EFBIG: xmlParserErrors = 1_512;
pub const xmlParserErrors_XML_IO_EINPROGRESS: xmlParserErrors = 1_513;
pub const xmlParserErrors_XML_IO_EINTR: xmlParserErrors = 1_514;
pub const xmlParserErrors_XML_IO_EINVAL: xmlParserErrors = 1_515;
pub const xmlParserErrors_XML_IO_EIO: xmlParserErrors = 1_516;
pub const xmlParserErrors_XML_IO_EISDIR: xmlParserErrors = 1_517;
pub const xmlParserErrors_XML_IO_EMFILE: xmlParserErrors = 1_518;
pub const xmlParserErrors_XML_IO_EMLINK: xmlParserErrors = 1_519;
pub const xmlParserErrors_XML_IO_EMSGSIZE: xmlParserErrors = 1_520;
pub const xmlParserErrors_XML_IO_ENAMETOOLONG: xmlParserErrors = 1_521;
pub const xmlParserErrors_XML_IO_ENFILE: xmlParserErrors = 1_522;
pub const xmlParserErrors_XML_IO_ENODEV: xmlParserErrors = 1_523;
pub const xmlParserErrors_XML_IO_ENOENT: xmlParserErrors = 1_524;
pub const xmlParserErrors_XML_IO_ENOEXEC: xmlParserErrors = 1_525;
pub const xmlParserErrors_XML_IO_ENOLCK: xmlParserErrors = 1_526;
pub const xmlParserErrors_XML_IO_ENOMEM: xmlParserErrors = 1_527;
pub const xmlParserErrors_XML_IO_ENOSPC: xmlParserErrors = 1_528;
pub const xmlParserErrors_XML_IO_ENOSYS: xmlParserErrors = 1_529;
pub const xmlParserErrors_XML_IO_ENOTDIR: xmlParserErrors = 1_530;
pub const xmlParserErrors_XML_IO_ENOTEMPTY: xmlParserErrors = 1_531;
pub const xmlParserErrors_XML_IO_ENOTSUP: xmlParserErrors = 1_532;
pub const xmlParserErrors_XML_IO_ENOTTY: xmlParserErrors = 1_533;
pub const xmlParserErrors_XML_IO_ENXIO: xmlParserErrors = 1_534;
pub const xmlParserErrors_XML_IO_EPERM: xmlParserErrors = 1_535;
pub const xmlParserErrors_XML_IO_EPIPE: xmlParserErrors = 1_536;
pub const xmlParserErrors_XML_IO_ERANGE: xmlParserErrors = 1_537;
pub const xmlParserErrors_XML_IO_EROFS: xmlParserErrors = 1_538;
pub const xmlParserErrors_XML_IO_ESPIPE: xmlParserErrors = 1_539;
pub const xmlParserErrors_XML_IO_ESRCH: xmlParserErrors = 1_540;
pub const xmlParserErrors_XML_IO_ETIMEDOUT: xmlParserErrors = 1_541;
pub const xmlParserErrors_XML_IO_EXDEV: xmlParserErrors = 1_542;
pub const xmlParserErrors_XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1_543;
pub const xmlParserErrors_XML_IO_ENCODER: xmlParserErrors = 1_544;
pub const xmlParserErrors_XML_IO_FLUSH: xmlParserErrors = 1_545;
pub const xmlParserErrors_XML_IO_WRITE: xmlParserErrors = 1_546;
pub const xmlParserErrors_XML_IO_NO_INPUT: xmlParserErrors = 1_547;
pub const xmlParserErrors_XML_IO_BUFFER_FULL: xmlParserErrors = 1_548;
pub const xmlParserErrors_XML_IO_LOAD_ERROR: xmlParserErrors = 1_549;
pub const xmlParserErrors_XML_IO_ENOTSOCK: xmlParserErrors = 1_550;
pub const xmlParserErrors_XML_IO_EISCONN: xmlParserErrors = 1_551;
pub const xmlParserErrors_XML_IO_ECONNREFUSED: xmlParserErrors = 1_552;
pub const xmlParserErrors_XML_IO_ENETUNREACH: xmlParserErrors = 1_553;
pub const xmlParserErrors_XML_IO_EADDRINUSE: xmlParserErrors = 1_554;
pub const xmlParserErrors_XML_IO_EALREADY: xmlParserErrors = 1_555;
pub const xmlParserErrors_XML_IO_EAFNOSUPPORT: xmlParserErrors = 1_556;
pub const xmlParserErrors_XML_XINCLUDE_RECURSION: xmlParserErrors = 1_600;
pub const xmlParserErrors_XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1_601;
pub const xmlParserErrors_XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1_602;
pub const xmlParserErrors_XML_XINCLUDE_NO_HREF: xmlParserErrors = 1_603;
pub const xmlParserErrors_XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1_604;
pub const xmlParserErrors_XML_XINCLUDE_HREF_URI: xmlParserErrors = 1_605;
pub const xmlParserErrors_XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1_606;
pub const xmlParserErrors_XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1_607;
pub const xmlParserErrors_XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1_608;
pub const xmlParserErrors_XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1_609;
pub const xmlParserErrors_XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1_610;
pub const xmlParserErrors_XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1_611;
pub const xmlParserErrors_XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1_612;
pub const xmlParserErrors_XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1_613;
pub const xmlParserErrors_XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1_614;
pub const xmlParserErrors_XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1_615;
pub const xmlParserErrors_XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1_616;
pub const xmlParserErrors_XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1_617;
pub const xmlParserErrors_XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1_618;
pub const xmlParserErrors_XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1_650;
pub const xmlParserErrors_XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1_651;
pub const xmlParserErrors_XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1_652;
pub const xmlParserErrors_XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1_653;
pub const xmlParserErrors_XML_CATALOG_RECURSION: xmlParserErrors = 1_654;
pub const xmlParserErrors_XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1_700;
pub const xmlParserErrors_XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1_701;
pub const xmlParserErrors_XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1_702;
pub const xmlParserErrors_XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1_703;
pub const xmlParserErrors_XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1_704;
pub const xmlParserErrors_XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1_705;
pub const xmlParserErrors_XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1_706;
pub const xmlParserErrors_XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1_707;
pub const xmlParserErrors_XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1_708;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1_709;
pub const xmlParserErrors_XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1_710;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1_711;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1_712;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1_713;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1_714;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1_715;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1_716;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1_717;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1_718;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1_719;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1_720;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1_721;
pub const xmlParserErrors_XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1_722;
pub const xmlParserErrors_XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1_723;
pub const xmlParserErrors_XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1_724;
pub const xmlParserErrors_XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1_725;
pub const xmlParserErrors_XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1_726;
pub const xmlParserErrors_XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1_727;
pub const xmlParserErrors_XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1_728;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1_729;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1_730;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1_731;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1_732;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1_733;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1_734;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1_735;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1_736;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1_737;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1_738;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1_739;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1_740;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1_741;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1_742;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1_743;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1_744;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1_745;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1_746;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1_747;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1_748;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1_749;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1_750;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1_751;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1_752;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1_753;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1_754;
pub const xmlParserErrors_XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1_755;
pub const xmlParserErrors_XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1_756;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1_757;
pub const xmlParserErrors_XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1_758;
pub const xmlParserErrors_XML_SCHEMAP_NOROOT: xmlParserErrors = 1_759;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1_760;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1_761;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1_762;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1_763;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1_764;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1_765;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1_766;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1_767;
pub const xmlParserErrors_XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1_768;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1_769;
pub const xmlParserErrors_XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1_770;
pub const xmlParserErrors_XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1_771;
pub const xmlParserErrors_XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1_772;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1_773;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1_774;
pub const xmlParserErrors_XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1_775;
pub const xmlParserErrors_XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1_776;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1_777;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1_778;
pub const xmlParserErrors_XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1_779;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1_780;
pub const xmlParserErrors_XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1_781;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1_782;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1_783;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1_784;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1_785;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1_786;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1_787;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1_788;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1_789;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1_790;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1_791;
pub const xmlParserErrors_XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1_792;
pub const xmlParserErrors_XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1_793;
pub const xmlParserErrors_XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1_794;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1_795;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1_796;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1_797;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1_798;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1_799;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1_800;
pub const xmlParserErrors_XML_SCHEMAV_NOROOT: xmlParserErrors = 1_801;
pub const xmlParserErrors_XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1_802;
pub const xmlParserErrors_XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1_803;
pub const xmlParserErrors_XML_SCHEMAV_MISSING: xmlParserErrors = 1_804;
pub const xmlParserErrors_XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1_805;
pub const xmlParserErrors_XML_SCHEMAV_NOTYPE: xmlParserErrors = 1_806;
pub const xmlParserErrors_XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1_807;
pub const xmlParserErrors_XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1_808;
pub const xmlParserErrors_XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1_809;
pub const xmlParserErrors_XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1_810;
pub const xmlParserErrors_XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1_811;
pub const xmlParserErrors_XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1_812;
pub const xmlParserErrors_XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1_813;
pub const xmlParserErrors_XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1_814;
pub const xmlParserErrors_XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1_815;
pub const xmlParserErrors_XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1_816;
pub const xmlParserErrors_XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1_817;
pub const xmlParserErrors_XML_SCHEMAV_INTERNAL: xmlParserErrors = 1_818;
pub const xmlParserErrors_XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1_819;
pub const xmlParserErrors_XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1_820;
pub const xmlParserErrors_XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1_821;
pub const xmlParserErrors_XML_SCHEMAV_VALUE: xmlParserErrors = 1_822;
pub const xmlParserErrors_XML_SCHEMAV_FACET: xmlParserErrors = 1_823;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1_824;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1_825;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1_826;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1_827;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1_828;
pub const xmlParserErrors_XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1_829;
pub const xmlParserErrors_XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1_830;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1_831;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1_832;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1_833;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1_834;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1_835;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1_836;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1_837;
pub const xmlParserErrors_XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1_838;
pub const xmlParserErrors_XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1_839;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1_840;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1_841;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1_842;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1_843;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1_844;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1_845;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1_846;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1_847;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1_848;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1_849;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1_850;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1_851;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1_852;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1_853;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1_854;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1_855;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1_856;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1_857;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1_858;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1_859;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1_860;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1_861;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1_862;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1_863;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1_864;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1_865;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1_866;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1_867;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1_868;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1_869;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1_870;
pub const xmlParserErrors_XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1_871;
pub const xmlParserErrors_XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1_872;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1_873;
pub const xmlParserErrors_XML_SCHEMAV_CVC_AU: xmlParserErrors = 1_874;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1_875;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1_876;
pub const xmlParserErrors_XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1_877;
pub const xmlParserErrors_XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1_878;
pub const xmlParserErrors_XML_SCHEMAV_MISC: xmlParserErrors = 1_879;
pub const xmlParserErrors_XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1_900;
pub const xmlParserErrors_XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1_901;
pub const xmlParserErrors_XML_XPTR_EVAL_FAILED: xmlParserErrors = 1_902;
pub const xmlParserErrors_XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1_903;
pub const xmlParserErrors_XML_C14N_CREATE_CTXT: xmlParserErrors = 1_950;
pub const xmlParserErrors_XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1_951;
pub const xmlParserErrors_XML_C14N_CREATE_STACK: xmlParserErrors = 1_952;
pub const xmlParserErrors_XML_C14N_INVALID_NODE: xmlParserErrors = 1_953;
pub const xmlParserErrors_XML_C14N_UNKNOW_NODE: xmlParserErrors = 1_954;
pub const xmlParserErrors_XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1_955;
pub const xmlParserErrors_XML_FTP_PASV_ANSWER: xmlParserErrors = 2_000;
pub const xmlParserErrors_XML_FTP_EPSV_ANSWER: xmlParserErrors = 2_001;
pub const xmlParserErrors_XML_FTP_ACCNT: xmlParserErrors = 2_002;
pub const xmlParserErrors_XML_FTP_URL_SYNTAX: xmlParserErrors = 2_003;
pub const xmlParserErrors_XML_HTTP_URL_SYNTAX: xmlParserErrors = 2_020;
pub const xmlParserErrors_XML_HTTP_USE_IP: xmlParserErrors = 2_021;
pub const xmlParserErrors_XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2_022;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3_000;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3_001;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3_002;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3_003;
pub const xmlParserErrors_XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3_004;
pub const xmlParserErrors_XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors = 3_005;
pub const xmlParserErrors_XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3_006;
pub const xmlParserErrors_XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors = 3_007;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3_008;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3_009;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3_010;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3_011;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3_012;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3_013;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3_014;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3_015;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3_016;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3_017;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3_018;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3_019;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3_020;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3_021;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3_022;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3_023;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3_024;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3_025;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3_026;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3_027;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3_028;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3_029;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3_030;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3_031;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3_032;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3_033;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3_034;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3_035;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3_036;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3_037;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3_038;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3_039;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3_040;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3_041;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3_042;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3_043;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3_044;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3_045;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3_046;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3_047;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3_048;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3_049;
pub const xmlParserErrors_XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3_050;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3_051;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3_052;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3_053;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3_054;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3_055;
pub const xmlParserErrors_XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3_056;
pub const xmlParserErrors_XML_SCHEMAP_NO_XSI: xmlParserErrors = 3_057;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3_058;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3_059;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3_060;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3_061;
pub const xmlParserErrors_XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3_062;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3_063;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3_064;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3_065;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3_066;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3_067;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3_068;
pub const xmlParserErrors_XML_SCHEMAP_INTERNAL: xmlParserErrors = 3_069;
pub const xmlParserErrors_XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3_070;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3_071;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3_072;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3_073;
pub const xmlParserErrors_XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3_074;
pub const xmlParserErrors_XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3_075;
pub const xmlParserErrors_XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3_076;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3_077;
pub const xmlParserErrors_XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3_078;
pub const xmlParserErrors_XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3_079;
pub const xmlParserErrors_XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3_080;
pub const xmlParserErrors_XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3_081;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3_082;
pub const xmlParserErrors_XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3_083;
pub const xmlParserErrors_XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3_084;
pub const xmlParserErrors_XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3_085;
pub const xmlParserErrors_XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3_086;
pub const xmlParserErrors_XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3_087;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3_088;
pub const xmlParserErrors_XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3_089;
pub const xmlParserErrors_XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3_090;
pub const xmlParserErrors_XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3_091;
pub const xmlParserErrors_XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4_000;
pub const xmlParserErrors_XML_SCHEMATRONV_REPORT: xmlParserErrors = 4_001;
pub const xmlParserErrors_XML_MODULE_OPEN: xmlParserErrors = 4_900;
pub const xmlParserErrors_XML_MODULE_CLOSE: xmlParserErrors = 4_901;
pub const xmlParserErrors_XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5_000;
pub const xmlParserErrors_XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5_001;
pub const xmlParserErrors_XML_CHECK_FOUND_TEXT: xmlParserErrors = 5_002;
pub const xmlParserErrors_XML_CHECK_FOUND_CDATA: xmlParserErrors = 5_003;
pub const xmlParserErrors_XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5_004;
pub const xmlParserErrors_XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5_005;
pub const xmlParserErrors_XML_CHECK_FOUND_PI: xmlParserErrors = 5_006;
pub const xmlParserErrors_XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5_007;
pub const xmlParserErrors_XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5_008;
pub const xmlParserErrors_XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5_009;
pub const xmlParserErrors_XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5_010;
pub const xmlParserErrors_XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5_011;
pub const xmlParserErrors_XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5_012;
pub const xmlParserErrors_XML_CHECK_NO_PARENT: xmlParserErrors = 5_013;
pub const xmlParserErrors_XML_CHECK_NO_DOC: xmlParserErrors = 5_014;
pub const xmlParserErrors_XML_CHECK_NO_NAME: xmlParserErrors = 5_015;
pub const xmlParserErrors_XML_CHECK_NO_ELEM: xmlParserErrors = 5_016;
pub const xmlParserErrors_XML_CHECK_WRONG_DOC: xmlParserErrors = 5_017;
pub const xmlParserErrors_XML_CHECK_NO_PREV: xmlParserErrors = 5_018;
pub const xmlParserErrors_XML_CHECK_WRONG_PREV: xmlParserErrors = 5_019;
pub const xmlParserErrors_XML_CHECK_NO_NEXT: xmlParserErrors = 5_020;
pub const xmlParserErrors_XML_CHECK_WRONG_NEXT: xmlParserErrors = 5_021;
pub const xmlParserErrors_XML_CHECK_NOT_DTD: xmlParserErrors = 5_022;
pub const xmlParserErrors_XML_CHECK_NOT_ATTR: xmlParserErrors = 5_023;
pub const xmlParserErrors_XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5_024;
pub const xmlParserErrors_XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5_025;
pub const xmlParserErrors_XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5_026;
pub const xmlParserErrors_XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5_027;
pub const xmlParserErrors_XML_CHECK_NO_HREF: xmlParserErrors = 5_028;
pub const xmlParserErrors_XML_CHECK_WRONG_PARENT: xmlParserErrors = 5_029;
pub const xmlParserErrors_XML_CHECK_NS_SCOPE: xmlParserErrors = 5_030;
pub const xmlParserErrors_XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5_031;
pub const xmlParserErrors_XML_CHECK_NOT_UTF8: xmlParserErrors = 5_032;
pub const xmlParserErrors_XML_CHECK_NO_DICT: xmlParserErrors = 5_033;
pub const xmlParserErrors_XML_CHECK_NOT_NCNAME: xmlParserErrors = 5_034;
pub const xmlParserErrors_XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5_035;
pub const xmlParserErrors_XML_CHECK_WRONG_NAME: xmlParserErrors = 5_036;
pub const xmlParserErrors_XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5_037;
pub const xmlParserErrors_XML_I18N_NO_NAME: xmlParserErrors = 6_000;
pub const xmlParserErrors_XML_I18N_NO_HANDLER: xmlParserErrors = 6_001;
pub const xmlParserErrors_XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6_002;
pub const xmlParserErrors_XML_I18N_CONV_FAILED: xmlParserErrors = 6_003;
pub const xmlParserErrors_XML_I18N_NO_OUTPUT: xmlParserErrors = 6_004;
pub const xmlParserErrors_XML_BUF_OVERFLOW: xmlParserErrors = 7000;

/// Signature of the function to use when there is an error and no parsing or validity context available .
/// ctx:	a parsing context
/// msg:	the message
/// ...:	the extra arguments of the varargs to format the message
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(ctx: *mut void, msg: *const c_char, ...)>;

/// Signature of the function to use when there is an error and the module handles the new error reporting mechanism.
/// userData:	user provided data for the error callback
/// error:	the error being raised.
pub type xmlStructuredErrorFunc = Option<unsafe fn(userData: *mut void, error: xmlErrorPtr)>;

extern "C" {
    /// Set or reset (if NULL) the default handler for generic errors to the builtin error function.
    /// andler:	the handler
    pub fn initGenericErrorDefaultFunc(handler: *mut xmlGenericErrorFunc);
}

extern "C" {
    /// Save the original error to the new place.
    /// from:	a source error
    /// to:	a target error
    /// Returns:	0 in case of success and -1 in case of error.
    pub fn xmlCopyError(form: xmlErrorPtr, to: xmlErrorPtr);
}

extern "C" {
    /// Get the last parsing error registered.
    /// ctx:	an XML parser context
    /// Returns:	NULL if no error occurred or a pointer to the error
    pub fn xmlCtxtGetLastError(ctx: *mut void) -> xmlErrorPtr;
}

extern "C" {
    /// Cleanup the last global error registered. For parsing error this does not change the well-formedness result.
    /// ctx:	an XML parser context
    pub fn xmlCtxtResetLastError(ctx: *mut void);
}

extern "C" {
    /// Get the last global error registered. This is per thread if compiled with thread support.
    /// Returns:	NULL if no error occurred or a pointer to the error
    pub fn xmlGetLastError() -> xmlErrorPtr;
}

extern "C" {
    /// Display and format an error messages, gives file, line, position and extra parameters.
    /// ctx:	an XML parser context
    /// msg:	the message to display/transmit
    /// ...:	extra parameters for the message display
    pub fn xmlParserError(ctx: *mut void, msg: *const c_char, ...);
}

extern "C" {
    /// Displays current context within the input content for error tracking
    /// input:	an xmlParserInputPtr input
    pub fn xmlParserPrintFileContext(input: xmlParserInputPtr);
}

extern "C" {
    /// Displays the associated file and line information for the current input
    /// input:	an xmlParserInputPtr input
    pub fn xmlParserPrintFileInfo(input: xmlParserInputPtr);
}

extern "C" {
    /// Display and format an validity error messages, gives file, line, position and extra parameters.
    /// ctx:	an XML parser context
    /// msg:	the message to display/transmit
    /// ...:	extra parameters for the message display
    pub fn xmlParserValidityError(ctx: *mut void, msg: *const c_char, ...);
}

extern "C" {
    /// Display and format a validity warning messages, gives file, line, position and extra parameters.
    /// ctx:	an XML parser context
    /// msg:	the message to display/transmit
    /// ...:	extra parameters for the message display
    pub fn xmlParserValidityWarning(ctx: *mut void, msg: *const c_char, ...);
}

extern "C" {
    /// Display and format a warning messages, gives file, line, position and extra parameters.
    /// ctx:	an XML parser context
    /// msg:	the message to display/transmit
    /// ...:	extra parameters for the message display
    pub fn xmlParserWarning(ctx: *mut void, msg: *const c_char, ...);
}

extern "C" {
    /// Cleanup the error.
    /// err:	pointer to the error.
    pub fn xmlResetError(err: xmlErrorPtr);
}

extern "C" {
    /// Cleanup the last global error registered. For parsing error this does not change the well-formedness result.
    pub fn xmlResetLastError();
}

extern "C" {
    /// Function to reset the handler and the error context for out of context error messages. This simply means that @handler will be called for subsequent error messages while not parsing nor validating. And @ctx will be passed as first argument to @handler One can simply force messages to be emitted to another FILE * than stderr by setting @ctx to this file handle and @handler to NULL. For multi-threaded applications, this must be set separately for each thread.
    /// ctx:	the new error handling context
    /// handler:	the new handler function
    pub fn xmlSetGenericErrorFunc(ctx: *mut void, handler: xmlGenericErrorFunc);
}

extern "C" {
    /// Function to reset the handler and the error context for out of context structured error messages. This simply means that @handler will be called for subsequent error messages while not parsing nor validating. And @ctx will be passed as first argument to @handler For multi-threaded applications, this must be set separately for each thread.
    /// ctx:	the new error handling context
    /// handler:	the new handler function
    pub fn xmlSetStructuredErrorFunc(ctx: *mut void, handler: xmlStructuredErrorFunc);
}
