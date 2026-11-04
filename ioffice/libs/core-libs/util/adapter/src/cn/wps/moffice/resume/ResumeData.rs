// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName
// Java: import java.util.ArrayList
// Java: import java.util.List

use super::BaseInfo::BaseInfo;
use super::JobIntention::JobIntention;
use super::Experience::Experience;
use super::Intern::Intern;
use super::Education::Education;
use super::ProgramExperience::ProgramExperience;
use super::Qualifications::Qualifications;
use super::Extra::Extra;
use super::Module::Module;

// Java field: String FLAG_IS_HIDE_TEMPLATE
pub(crate) const FLAG_IS_HIDE_TEMPLATE: &str = "\u{3000}";

// Java: public class ResumeData
pub struct ResumeData {
    // Java field: BaseInfo mBaseInfo
    pub(crate) mBaseInfo: Option<BaseInfo>,
    // Java field: JobIntention mJobIntention
    pub(crate) mJobIntention: Option<JobIntention>,
    // Java field: List<Experience> mExperience
    pub(crate) mExperience: Option<Vec<Experience>>,
    // Java field: List<Intern> mIntern
    pub(crate) mIntern: Option<Vec<Intern>>,
    // Java field: List<SchoolExp> mSchoolExp
    pub(crate) mSchoolExp: Option<Vec<Box<dyn std::any::Any>>>,
    // Java field: List<ProgramExperience> mProgramExperience
    pub(crate) mProgramExperience: Option<Vec<ProgramExperience>>,
    // Java field: List<Education> mEducation
    pub(crate) mEducation: Option<Vec<Education>>,
    // Java field: String mSkillCertificate
    pub(crate) mSkillCertificate: Option<String>,
    // Java field: String mSelfEvaluation
    pub(crate) mSelfEvaluation: Option<String>,
    // Java field: Qualifications mQualifications
    pub(crate) mQualifications: Option<Qualifications>,
    // Java field: Extra mExtra
    pub(crate) mExtra: Option<Extra>,
    // Java field: ArrayList<Module> mModules
    pub(crate) mModules: Option<Vec<Module>>,
    // Java field: String mTemplatePath
    pub(crate) mTemplatePath: Option<String>,
    // Java field: ArrayList<Module> mPCModules
    pub(crate) mPCModules: Option<Vec<Module>>,
    // Java field: boolean mSkipReorder
    mSkipReorder: bool,
    // Java field: ArrayList<String> mNewOrders
    pub(crate) mNewOrders: Option<Vec<String>>,
}

// Java: public class ResumeData
impl ResumeData {
    // Java: ResumeData()
    pub fn new() -> Self {
      // Java: super();
      ResumeData {
          mBaseInfo: None,
          mJobIntention: None,
          mExperience: None,
          mIntern: None,
          mSchoolExp: None,
          mProgramExperience: None,
          mEducation: None,
          mSkillCertificate: None,
          mSelfEvaluation: None,
          mQualifications: None,
          mExtra: None,
          mModules: None,
          mTemplatePath: None,
          mPCModules: None,
          mSkipReorder: false,
          mNewOrders: None,
      }
    }

    // Java: void setSkipReorder(boolean skipReorder)
    pub fn setSkipReorder(&mut self, _skipReorder: bool) {
      // Java: this.mSkipReorder = skipReorder;
      self.mSkipReorder = _skipReorder;
    }

    // Java: ArrayList<Module> getPCModules()
    pub fn getPCModules(&self) -> &Option<Vec<Module>> {
      // Java: return this.mPCModules;
      &self.mPCModules
    }

    // Java: void setPCModules(ArrayList<Module> pcModules)
    pub fn setPCModules(&mut self, _pcModules: Vec<Module>) {
      // Java: this.mPCModules = pcModules;
      self.mPCModules = Some(_pcModules);
    }

    // Java: ArrayList<Module> getModules()
    pub fn getModules(&self) -> &Option<Vec<Module>> {
      // Java: return this.mModules;
      &self.mModules
    }

    // Java: void setModules(ArrayList<Module> modules)
    pub fn setModules(&mut self, _modules: Vec<Module>) {
      // Java: this.mModules = modules;
      self.mModules = Some(_modules);
    }

    // Java: ArrayList<String> getNewOrders()
    pub fn getNewOrders(&self) -> &Option<Vec<String>> {
      // Java: return this.mNewOrders;
      &self.mNewOrders
    }

    // Java: void setNewOrders(ArrayList<String> newOrders)
    pub fn setNewOrders(&mut self, _newOrders: Vec<String>) {
      // Java: this.mNewOrders = newOrders;
      self.mNewOrders = Some(_newOrders);
    }

    // Java: String getTemplatePath()
    pub fn getTemplatePath(&self) -> Option<&str> {
      // Java: return this.mTemplatePath;
      self.mTemplatePath.as_deref()
    }

    // Java: void setTemplatePath(String templatePath)
    pub fn setTemplatePath(&mut self, _templatePath: &str) {
      // Java: this.mTemplatePath = templatePath;
      self.mTemplatePath = Some(_templatePath.to_string());
    }

    // Java: BaseInfo getBaseInfo()
    pub fn getBaseInfo(&self) -> &Option<BaseInfo> {
      // Java: return this.mBaseInfo;
      &self.mBaseInfo
    }

    // Java: JobIntention getJobIntention()
    pub fn getJobIntention(&self) -> &Option<JobIntention> {
      // Java: return this.mJobIntention;
      &self.mJobIntention
    }

    // Java: List<Experience> getExperience()
    pub fn getExperience(&self) -> &Option<Vec<Experience>> {
      // Java: return this.mExperience;
      &self.mExperience
    }

    // Java: List<Education> getEducation()
    pub fn getEducation(&self) -> &Option<Vec<Education>> {
      // Java: return this.mEducation;
      &self.mEducation
    }

    // Java: List<Intern> getIntern()
    pub fn getIntern(&self) -> &Option<Vec<Intern>> {
      // Java: return this.mIntern;
      &self.mIntern
    }

    // Java: List<SchoolExp> getSchoolExp()
    pub fn getSchoolExp(&self) -> &Option<Vec<Box<dyn std::any::Any>>> {
      // Java: return this.mSchoolExp;
      &self.mSchoolExp
    }

    // Java: String getSkillCertificate()
    pub fn getSkillCertificate(&self) -> Option<&str> {
      // Java: return this.mSkillCertificate;
      self.mSkillCertificate.as_deref()
    }

    // Java: String getSelfEvaluation()
    pub fn getSelfEvaluation(&self) -> Option<&str> {
      // Java: return this.mSelfEvaluation;
      self.mSelfEvaluation.as_deref()
    }

    // Java: List<ProgramExperience> getProgramExperience()
    pub fn getProgramExperience(&self) -> &Option<Vec<ProgramExperience>> {
      // Java: return this.mProgramExperience;
      &self.mProgramExperience
    }

    // Java: Extra getExtra()
    pub fn getExtra(&self) -> &Option<Extra> {
      // Java: return this.mExtra;
      &self.mExtra
    }

    // Java: Qualifications getQualifications()
    pub fn getQualifications(&self) -> &Option<Qualifications> {
      // Java: return this.mQualifications;
      &self.mQualifications
    }

    // Java: boolean isEmptyResumeData()
    pub fn isEmptyResumeData(&self) -> bool {
      // Java: return this.mBaseInfo == null && this.mIntern == null && this.mSchoolExp == null && this.mQualifications == null && this.mJobIntention == null && this.mExperience == null && this.mEducation == null && this.mProgramExperience == null && this.mSkillCertificate == null && this.mSelfEvaluation == null && this.mExtra == null;
      self.mBaseInfo.is_none() && self.mIntern.is_none() && self.mSchoolExp.is_none() && self.mQualifications.is_none() && self.mJobIntention.is_none() && self.mExperience.is_none() && self.mEducation.is_none() && self.mProgramExperience.is_none() && self.mSkillCertificate.is_none() && self.mSelfEvaluation.is_none() && self.mExtra.is_none()
    }

    // Java: ResumeData getRenderData()
    pub fn getRenderData(&self) -> ResumeData {
      // Java: ResumeData data = new ResumeData();
      let mut data = ResumeData::new();
      // Java: if (this.mBaseInfo != null && !this.mBaseInfo.mIsHideInTemplate)
      if let Some(ref base_info) = self.mBaseInfo {
          if !base_info.base.mIsHideInTemplate {
              // Java: data.mBaseInfo = this.mBaseInfo;
          }
      }
      // Java: if (this.mJobIntention != null && !this.mJobIntention.mIsHideInTemplate)
      if let Some(ref job) = self.mJobIntention {
          if !job.base.mIsHideInTemplate {
              // Java: data.mJobIntention = this.mJobIntention;
          }
      }
      // Java: if (this.mExperience != null && this.mExperience.size() > 0)
      if let Some(ref exps) = self.mExperience {
          if !exps.is_empty() {
              // Java: List<Experience> experiences = new ArrayList();
              // Java: for(Experience experience : this.mExperience)
              // Java: if (!experience.mIsHideInTemplate)
              // Java: experiences.add(experience);
              // Java: data.mExperience = experiences;
          }
      }
      // Java: if (this.mEducation != null && this.mEducation.size() > 0)
      if let Some(ref edus) = self.mEducation {
          if !edus.is_empty() {
              // Java: List<Education> educations = new ArrayList();
              // Java: for(Education education : this.mEducation)
              // Java: if (!education.mIsHideInTemplate)
              // Java: educations.add(education);
              // Java: data.mEducation = educations;
          }
      }
      // Java: if (this.mIntern != null && this.mIntern.size() > 0)
      if let Some(ref interns) = self.mIntern {
          if !interns.is_empty() {
              // Java: List<Intern> interns = new ArrayList();
              // Java: for(Intern intern : this.mIntern)
              // Java: if (!intern.mIsHideInTemplate)
              // Java: interns.add(intern);
              // Java: data.mIntern = interns;
          }
      }
      // Java: if (this.mSchoolExp != null && this.mSchoolExp.size() > 0)
      {
         // Java: List<SchoolExp> schoolExps = new ArrayList();
         // Java: for(SchoolExp schoolExp : this.mSchoolExp)
         // Java: if (!schoolExp.mIsHideInTemplate)
         // Java: schoolExps.add(schoolExp);
         // Java: data.mSchoolExp = schoolExps;
      }
      // Java: if (this.mProgramExperience != null && this.mProgramExperience.size() > 0)
      if let Some(ref progs) = self.mProgramExperience {
          if !progs.is_empty() {
              // Java: List<ProgramExperience> programExperiences = new ArrayList();
              // Java: for(ProgramExperience programExperience : this.mProgramExperience)
              // Java: if (!programExperience.mIsHideInTemplate)
              // Java: programExperiences.add(programExperience);
              // Java: data.mProgramExperience = programExperiences;
          }
      }
      // Java: if (this.mSkillCertificate != null && !this.mSkillCertificate.endsWith("　"))
      if let Some(ref skill) = self.mSkillCertificate {
          if !skill.ends_with('\u{3000}') {
              // Java: data.mSkillCertificate = this.mSkillCertificate;
              data.mSkillCertificate = self.mSkillCertificate.clone();
          }
      }
      // Java: if (this.mSelfEvaluation != null && !this.mSelfEvaluation.endsWith("　"))
      if let Some(ref eval) = self.mSelfEvaluation {
          if !eval.ends_with('\u{3000}') {
              // Java: data.mSelfEvaluation = this.mSelfEvaluation;
              data.mSelfEvaluation = self.mSelfEvaluation.clone();
          }
      }
      // Java: if (this.mQualifications != null && !this.mQualifications.mIsHideInTemplate)
      if let Some(ref quals) = self.mQualifications {
          if !quals.base.mIsHideInTemplate {
              // Java: data.mQualifications = this.mQualifications;
          }
      }
      // Java: if (this.mExtra != null && this.mExtra.getCustomModule() != null && !this.mExtra.getCustomModule().mIsHideInTemplate)
      {
         // Java: data.mExtra = new Extra();
         // Java: data.mExtra.mCustomModule = this.mExtra.getCustomModule();
      }
      // Java: data.mTemplatePath = this.mTemplatePath;
      data.mTemplatePath = self.mTemplatePath.clone();
      // Java: if (this.mModules != null)
      {
         // Java: data.setModules(this.mModules);
      }
      // Java: if (this.mNewOrders != null)
      {
         // Java: data.setNewOrders(this.mNewOrders);
      }
      // Java: data.mSkipReorder = this.mSkipReorder;
      data.mSkipReorder = self.mSkipReorder;
      // Java: return data;
      data
    }

    // Java: boolean needReorderResume()
    pub fn needReorderResume(&self) -> bool {
      // Java: return null != this.mNewOrders && !this.mNewOrders.isEmpty() && !this.mSkipReorder;
      self.mNewOrders.as_ref().map_or(false, |o| !o.is_empty()) && !self.mSkipReorder
    }

}
