#![allow(non_snake_case)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.writer.data.value.DateTime
use crate::cn::wps::moffice::writer::data::value::DateTime::DateTime;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import java.security.SecureRandom
// Rust draws each int from the platform CSPRNG, as Java SecureRandom does.
// Java: import java.util.ArrayList
// Java: import java.util.Date
use chrono::{Datelike, Local, Timelike};
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};

// java private static SecureRandom mRandam = new SecureRandom();
// UUID v4 draws OS/JS cryptographic randomness. Its first 32 bits are
// unconstrained random bits, matching SecureRandom.nextInt's value domain.

// java public class KRevision
#[derive(Default)]
pub struct KRevision {
    // java TextDocument mTextDocument = null;
    mTextDocument: Option<*const (dyn SoftTextDocument + 'static)>,
}

// java public class KRevision
impl KRevision {
    // java KRevision(TextDocument textDocument)
    pub fn KRevision(textDocument: &dyn SoftTextDocument) -> Self {
        // java Assert.assertNotNull("textDocument should not be null !", textDocument);
        Assert::assertNotNull_msg("textDocument should not be null !", Some(textDocument as *const dyn SoftTextDocument as *const ()));
        // java this.mTextDocument = textDocument;
        Self {
            mTextDocument: Some(unsafe {
                std::mem::transmute::<*const dyn SoftTextDocument, *const (dyn SoftTextDocument + 'static)>(
                    textDocument as *const dyn SoftTextDocument,
                )
            }),
        }
    }

    fn textDocument(&self) -> &dyn SoftTextDocument {
        unsafe { &*self.mTextDocument.expect("KRevision.mTextDocument is null") }
    }

    // java int assignRsid()
    pub fn assignRsid(&self) -> i32 {
        // java int rsid = mRandam.nextInt();
        let random = uuid::Uuid::new_v4();
        let rsid = i32::from_ne_bytes(random.as_bytes()[0..4].try_into().expect("UUID prefix"));
        // java TextDocument textDocument = this.mTextDocument;
        let textDocument = self.textDocument();
        // java Assert.assertNotNull("textDocument should not be null", textDocument);
        Assert::assertNotNull_msg("textDocument should not be null", Some(textDocument as *const dyn SoftTextDocument as *const ()));
        // java ArrayList<Integer> rsids = textDocument.getRsids();
        let mut rsids = textDocument.getRsids();
        // java Assert.assertNotNull("rsids should not be null", rsids);
        Assert::assertNotNull_msg("rsids should not be null", rsids.as_ref());
        // java rsids.add(rsid);
        rsids.as_mut().expect("KRevision.getRsids returned null").push(rsid);
        // java return rsid;
        rsid
    }

    // java void setRevision(Revision revision)
    pub fn setRevision(&self, revision: &mut Revision) {
        // java Assert.assertNotNull("revision should not be null", revision);
        Assert::assertNotNull_msg("revision should not be null", Some(&*revision));
        // java Date date = new Date();
        let date = Local::now();
        // java String author = this.mTextDocument.getRevisionsAuthor();
        let author = self.textDocument().getRevisionsAuthor();
        // java Assert.logNotNull("author should not be null", author);
        Assert::logNotNull_msg("author should not be null", author.as_ref());
        // java Assert.logTrue("author.length() > 0 should true! ", null != author && author.length() > 0);
        Assert::logTrue_msg("author.length() > 0 should true! ", author.as_ref().is_some_and(|name| !name.is_empty()));
        // java DateTime dateTime = new DateTime(date.getMinutes(), date.getHours(), date.getDate(), date.getMonth() + 1, date.getYear() + 1900, date.getDay());
        let dateTime = DateTime::DateTime__int_int_int_int_int_int(
            date.minute() as i32,
            date.hour() as i32,
            date.day() as i32,
            date.month() as i32,
            date.year(),
            date.weekday().num_days_from_sunday() as i32,
        );
        // java revision.setDateTime(dateTime);
        revision.setDateTime(Some(dateTime));
        // java if (null != author)
        if let Some(author) = author {
            // java revision.setAuthor(author);
            revision.setAuthor(Some(author));
        }
    }

    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Option<&dyn Document> {
        // java return this.mTextDocument.getMainDocument();
        self.textDocument().getMainDocument()
    }
}
