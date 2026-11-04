// Java: package cn.wps.moffice.writer.data.value;
// Java: import cn.wps.base.assertion.Assert;
// Java: import cn.wps.moffice.property.PropertySet;
// Java: public class Revision implements Cloneable
use crate::cn::wps::moffice::writer::data::value::DateTime::DateTime;
use util::moffice::property::PropertySet::PropertySet;
// java import cn.wps.moffice.property.PropertySet 的静态成员 `empty`
use util::moffice::property::PropertySet::empty;

#[derive(Default, Clone)]
pub struct Revision {
    // java private int mark;
    pub mark: i32,
    // java private DateTime dateTime;
    pub dateTime: Option<DateTime>,
    // java private String author;
    pub author: Option<String>,
    // java private PropertySet property;
    pub property: Option<PropertySet>,
}

impl Revision {
    // java public static final int NONE = 0;
    pub const NONE: i32 = 0;
    // java public static final int DELETE = 1;
    pub const DELETE: i32 = 1;
    // java public static final int INSERT = 2;
    pub const INSERT: i32 = 2;
    // java public static final int FORMATCHANGE = 3;
    pub const FORMATCHANGE: i32 = 3;
    // java public static final String DEFAULT_AUTHOR = "Unknown";
    pub const DEFAULT_AUTHOR: &'static str = "Unknown";

    // java public Revision(int mark)
    pub fn Revision__int(mark: i32) -> Self {
      // java this(mark, "Unknown", new DateTime());
      Self::Revision__int_string_datetime(mark, "Unknown", Some(DateTime::DateTime()))
    }

    // java public Revision(int mark, String author, DateTime dateTime)
    pub fn Revision__int_string_datetime(mark: i32, author: &str, dateTime: Option<DateTime>) -> Self {
      // java this.mark = 0;
      // java this.dateTime = null;
      // java this.author = null;
      // java this.property = PropertySet.empty;
      // java this.mark = mark;
      // java this.author = author;
      // java this.dateTime = dateTime;
      Self { mark, dateTime, author: Some(author.to_string()), property: Some(empty.clone()) }
    }

    // java public int getMark()
    pub fn getMark(&self) -> i32 {
      // java return this.mark;
      return self.mark;
    }

    // java public void setMark(int mark)
    pub fn setMark(&mut self, mark: i32) {
      // java this.mark = mark;
      self.mark = mark;
    }

    // java public DateTime getDateTime()
    pub fn getDateTime(&self) -> Option<DateTime> {
      // java return this.dateTime;
      return self.dateTime.clone();
    }

    // java public void setDateTime(DateTime dateTime)
    pub fn setDateTime(&mut self, dateTime: Option<DateTime>) {
      // java this.dateTime = dateTime;
      self.dateTime = dateTime;
    }

    // java public String getAuthor()
    pub fn getAuthor(&self) -> Option<String> {
      // java return this.author;
      return self.author.clone();
    }

    // java public void setAuthor(String author)
    pub fn setAuthor(&mut self, author: Option<String>) {
      // java this.author = author;
      self.author = author;
    }

    // java public PropertySet getProperty()
    pub fn getProperty(&self) -> Option<PropertySet> {
      // java return this.property;
      return self.property.clone();
    }

    // java public void setProperty(PropertySet property)
    pub fn setProperty(&mut self, property: Option<PropertySet>) {
      // java Assert.assertNotNull("property should not be null!", property);
      util::base::assertion::Assert::Assert::assertNotNull_msg("property should not be null!", &property);
      // java this.property = property;
      self.property = property;
    }

    // java public boolean equals(Object revision)
    pub fn equals(&self, revision: Option<&Revision>) -> bool {
      // java if (revision != null && revision instanceof Revision)
      if let Some(srcRevision) = revision {
        // java Revision srcRevision = (Revision)revision;
        // java if (!this.hasSameAuthorAndContentWith(srcRevision))
        if !self.hasSameAuthorAndContentWith(Some(srcRevision)) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java DateTime srcDateTime = srcRevision.dateTime;
          let srcDateTime = srcRevision.dateTime.clone();
          // java DateTime dateTime = this.dateTime;
          let dateTime = self.dateTime.clone();
          // java if (null != srcDateTime && !srcDateTime.equals(dateTime))
          if let Some(ref sd) = srcDateTime {
            if !sd.equals(dateTime.as_ref()) {
              // java return false;
              return false;
            }
          }
          // java else
          // java return null == dateTime || dateTime.equals(srcDateTime);
          return dateTime.is_none() || dateTime.as_ref().map(|d| d.equals(srcDateTime.as_ref())).unwrap_or(false);
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java int h = this.mark;
      let mut h = self.mark;
      // java if (this.dateTime != null)
      if let Some(dt) = self.dateTime.as_ref() {
        // java h += this.dateTime.hashCode();
        h += dt.hashCode();
      }

      // java if (this.property != null)
      if let Some(p) = self.property.as_ref() {
        // java h += this.property.hashCode();
        let mut p = p.clone();
        h += p.hashCode();
      }

      // java if (this.author != null)
      if let Some(a) = self.author.as_ref() {
        // java h += this.author.hashCode();
        h += crate::cn::wps::moffice::writer::data::value::Revision::java_string_hashcode(a);
      }

      // java return h;
      return h;
    }

    // java public boolean hasSameAuthorAndContentWith(Revision srcRevision)
    pub fn hasSameAuthorAndContentWith(&self, srcRevision: Option<&Revision>) -> bool {
      // java if (srcRevision == null)
      let Some(srcRevision) = srcRevision else {
        // java return false;
        return false;
      };
      // java else if (this.mark != srcRevision.mark)
      if self.mark != srcRevision.mark {
        // java return false;
        return false;
      }
      // java else
      else {
        // java String srcAuthor = srcRevision.author;
        let srcAuthor = srcRevision.author.clone();
        // java String author = this.author;
        let author = self.author.clone();
        // java if (null != srcAuthor && !srcAuthor.equals(author))
        if let Some(ref sa) = srcAuthor {
          if Some(sa) != author.as_ref() {
            // java return false;
            return false;
          }
        }
        // java else if (null != author && !author.equals(srcAuthor))
        else if let Some(ref a) = author {
          if Some(a) != srcAuthor.as_ref() {
            // java return false;
            return false;
          }
        }
        // java else
        // java return this.property.equals(srcRevision.property);
        return self.property.as_ref().map(|p| p.equals(srcRevision.property.as_ref())).unwrap_or(false);
      }
    }

    // java public Revision clone() throws CloneNotSupportedException
    pub fn clone_(&self) -> Revision {
      // java Revision revision = (Revision)super.clone();
      let mut revision = self.clone();
      // java revision.author = this.author;
      revision.author = self.author.clone();
      // java revision.mark = this.mark;
      revision.mark = self.mark;
      // java revision.dateTime = this.dateTime.clone();
      revision.dateTime = self.dateTime.as_ref().map(|d| d.clone_());
      // java Assert.assertNotNull("this.property should not be null!", this.property);
      util::base::assertion::Assert::Assert::assertNotNull_msg("this.property should not be null!", &self.property);
      // java revision.property = this.property.clone();
      revision.property = self.property.clone();
      // java return revision;
      return revision;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder stringBuilder = new StringBuilder();
      let mut stringBuilder = String::new();
      // java stringBuilder.append(" author = ");
      stringBuilder.push_str(" author = ");
      // java stringBuilder.append('"');
      stringBuilder.push('"');
      // java stringBuilder.append(this.author);
      stringBuilder.push_str(self.author.as_deref().unwrap_or(""));
      // java stringBuilder.append('"');
      stringBuilder.push('"');
      // java stringBuilder.append(" {");
      stringBuilder.push_str(" {");
      // java stringBuilder.append("\n");
      stringBuilder.push('\n');
      // java stringBuilder.append("\t");
      stringBuilder.push('\t');
      // java stringBuilder.append(this.property.toString());
      stringBuilder.push_str(&self.property.as_ref().map(|p| p.toString()).unwrap_or_default());
      // java stringBuilder.append("\t");
      stringBuilder.push('\t');
      // java stringBuilder.append("}");
      stringBuilder.push('}');
      // java return stringBuilder.toString();
      return stringBuilder;
    }
}

// rust: ★Java `String.hashCode()` 的忠实复现（`s[0]*31^(n-1) + ...`，i32 溢出回绕）。
//   独立成自由函数，供 `Revision::hashCode` 使用；`// java` 原文见调用点。
pub fn java_string_hashcode(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
