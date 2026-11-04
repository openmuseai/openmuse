// java package cn.wps.et.ss.formula.udf;
// java
// java import cn.wps.et.ss.formula.atp.AnalysisToolPak;
// java import cn.wps.et.ss.formula.functions.base.IFunction;
// java import java.util.ArrayList;
// java import java.util.Arrays;
// java import java.util.Collection;
// java
// java public class AggregatingUDFFinder implements IUDFFinder {
// java public static final IUDFFinder DEFAULT;
// java private final Collection<IUDFFinder> _usedToolPacks;
// java
// java public AggregatingUDFFinder(IUDFFinder... usedToolPacks) {
// java this._usedToolPacks = new ArrayList(usedToolPacks.length);
// java this._usedToolPacks.addAll(Arrays.asList(usedToolPacks));
// java }
// java
// java public IFunction findFunction(String name) {
// java for(IUDFFinder pack : this._usedToolPacks) {
// java IFunction evaluatorForFunction = pack.findFunction(name);
// java if (evaluatorForFunction != null) {
// java return evaluatorForFunction;
// java }
// java }
// java
// java return null;
// java }
// java
// java public void add(IUDFFinder toolPack) {
// java this._usedToolPacks.add(toolPack);
// java }
// java
// java static {
// java DEFAULT = new AggregatingUDFFinder(new IUDFFinder[]{AnalysisToolPak.instance});
// java }
// java }
use super::IUDFFinder::IUDFFinder;use super::super::{evaluator::EvalResult,functions::base::IFunction::IFunction,atp::AnalysisToolPak::AnalysisToolPak};use std::sync::{Arc,LazyLock};use crate::java_runtime::RefCell;pub struct AggregatingUDFFinder{usedToolPacks:RefCell<Vec<Arc<dyn IUDFFinder>>>}pub static DEFAULT:LazyLock<AggregatingUDFFinder>=LazyLock::new(||AggregatingUDFFinder::new(vec![Arc::new(AnalysisToolPak)]));impl AggregatingUDFFinder{pub fn new(packs:Vec<Arc<dyn IUDFFinder>>)->Self{Self{usedToolPacks:RefCell::new(packs)}}pub fn add(&self,pack:Arc<dyn IUDFFinder>){self.usedToolPacks.borrow_mut().push(pack);}}impl IUDFFinder for AggregatingUDFFinder{fn findFunction(&self,name:&str)->EvalResult<Option<Arc<dyn IFunction>>>{for pack in self.usedToolPacks.borrow().iter(){if let Some(fun)=pack.findFunction(name)?{return Ok(Some(fun));}}Ok(None)}}
