// java public abstract class FormatCondition
pub struct FormatCondition {test:i32,c:f64}
impl FormatCondition {
    // java TESTS maps <,<=,>,>=,=,==,!=,<> to six comparison implementations.
    pub fn getInstance(opString:&str,constStr:&str)->Self {
        let test=match opString{"<"=>0,"<="=>1,">"=>2,">="=>3,"="|"=="=>4,"!="|"<>"=>5,_=>panic!("Unknown test: {opString}")};
        Self{test,c:constStr.parse().expect("NumberFormatException")}
    }
    // java six anonymous FormatCondition.pass(double value) implementations.
    pub fn pass(&self,value:f64)->bool {match self.test{0=>value<self.c,1=>value<=self.c,2=>value>self.c,3=>value>=self.c,4=>value==self.c,5=>value!=self.c,_=>unreachable!()}}
}
