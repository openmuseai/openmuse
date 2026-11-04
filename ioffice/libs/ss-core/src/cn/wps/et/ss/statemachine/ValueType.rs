// java package cn.wps.et.ss.statemachine;
// java
// java public enum ValueType {
// java BOOLEAN,
// java ERROR,
// java FRACTION,
// java PERCENT,
// java SENTIC,
// java NUMBER,
// java CURRENCY,
// java DATE,
// java TIME,
// java DATETIME,
// java STRING,
// java HYPERLINK;
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ValueType{BOOLEAN,ERROR,FRACTION,PERCENT,SENTIC,NUMBER,CURRENCY,DATE,TIME,DATETIME,STRING,HYPERLINK}
