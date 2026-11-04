// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/command_format.h"

#include "base/files/file_util.h"
#include "gn/commands.h"
#include "gn/filesystem_utils.h"
#include "gn/setup.h"
#include "gn/test_with_scheduler.h"
#include "util/exe_path.h"
#include "util/test/test.h"

base::FilePath GetFormatTestDataDir() {
  base::FilePath source_root_path =
      GetExePath().DirName().Append(FILE_PATH_LITERAL("source_root.txt"));
  std::string source_root;
  CHECK(base::ReadFileToString(source_root_path, &source_root))
      << "Failed to read " << FilePathToUTF8(source_root_path);
  return UTF8ToFilePath(source_root);
}

using FormatTest = TestWithScheduler;

#define FORMAT_TEST(n) FORMAT_TEST_WITH_WIDTH(n, commands::kDefaultFormatWidth)

#define FORMAT_TEST_WITH_WIDTH(n, format_width)                                \
  TEST_F(FormatTest, n) {                                                      \
    ::Setup setup;                                                             \
    std::string input;                                                         \
    std::string out;                                                           \
    std::string expected;                                                      \
    base::FilePath src_dir = GetFormatTestDataDir();                           \
    ASSERT_TRUE(base::SetCurrentDirectory(src_dir))                            \
        << "Failed to set current directory to: " << FilePathToUTF8(src_dir);  \
    base::FilePath current_dir;                                                \
    base::GetCurrentDirectory(&current_dir);                                   \
    base::FilePath input_file =                                                \
        base::FilePath(FILE_PATH_LITERAL("src/gn/format_test_data/")           \
                           FILE_PATH_LITERAL(#n) FILE_PATH_LITERAL(".gn"));    \
    ASSERT_TRUE(base::ReadFileToString(input_file, &input))                    \
        << "Failed to read input file: " << FilePathToUTF8(input_file)         \
        << " (src_dir: " << FilePathToUTF8(src_dir)                            \
        << ", current_dir: " << FilePathToUTF8(current_dir) << ")";            \
    base::FilePath golden_file = base::FilePath(                               \
        FILE_PATH_LITERAL("src/gn/format_test_data/") FILE_PATH_LITERAL(#n)    \
            FILE_PATH_LITERAL(".golden"));                                     \
    ASSERT_TRUE(base::ReadFileToString(golden_file, &expected))                \
        << "Failed to read golden file: " << FilePathToUTF8(golden_file)       \
        << " (src_dir: " << FilePathToUTF8(src_dir)                            \
        << ", current_dir: " << FilePathToUTF8(current_dir) << ")";            \
    EXPECT_TRUE(commands::FormatStringToString(                                \
        input, commands::TreeDumpMode::kInactive, format_width, &out,          \
        nullptr))                                                              \
        << "FormatStringToString failed for " << FilePathToUTF8(input_file);   \
    EXPECT_EQ(expected, out)                                                   \
        << "Formatted output mismatch for " << FilePathToUTF8(input_file);     \
    /* Make sure formatting the output doesn't cause further changes. */       \
    std::string out_again;                                                     \
    EXPECT_TRUE(                                                               \
        commands::FormatStringToString(out, commands::TreeDumpMode::kInactive, \
                                       format_width, &out_again, nullptr))     \
        << "Re-formatting failed for " << FilePathToUTF8(input_file);          \
    ASSERT_EQ(out, out_again)                                                  \
        << "Re-formatted output mismatch for " << FilePathToUTF8(input_file);  \
    /* Make sure we can roundtrip to json without any changes. */              \
    std::string as_json;                                                       \
    std::string unused;                                                        \
    EXPECT_TRUE(commands::FormatStringToString(                                \
        out_again, commands::TreeDumpMode::kJSON, format_width, &unused,       \
        &as_json))                                                             \
        << "FormatStringToString (JSON) failed for "                           \
        << FilePathToUTF8(input_file);                                         \
    std::string rewritten;                                                     \
    EXPECT_TRUE(                                                               \
        commands::FormatJsonToString(as_json, format_width, &rewritten))       \
        << "FormatJsonToString failed for " << FilePathToUTF8(input_file);     \
    ASSERT_EQ(out, rewritten) << "JSON roundtrip output mismatch for "         \
                              << FilePathToUTF8(input_file);                   \
  }

// These are expanded out this way rather than a runtime loop so that
// --gtest_filter works as expected for individual test running.
FORMAT_TEST(001)
FORMAT_TEST(002)
FORMAT_TEST(003)
FORMAT_TEST(004)
FORMAT_TEST(005)
FORMAT_TEST(006)
FORMAT_TEST(007)
FORMAT_TEST(008)
FORMAT_TEST(009)
FORMAT_TEST(010)
FORMAT_TEST(011)
FORMAT_TEST(012)
FORMAT_TEST(013)
FORMAT_TEST(014)
FORMAT_TEST(015)
FORMAT_TEST(016)
FORMAT_TEST(017)
FORMAT_TEST(018)
FORMAT_TEST(019)
FORMAT_TEST(020)
FORMAT_TEST(021)
FORMAT_TEST(022)
FORMAT_TEST(023)
FORMAT_TEST(024)
FORMAT_TEST(025)
FORMAT_TEST(026)
FORMAT_TEST(027)
FORMAT_TEST(028)
FORMAT_TEST(029)
FORMAT_TEST(030)
FORMAT_TEST(031)
FORMAT_TEST(032)
FORMAT_TEST(033)
// TODO(scottmg): args+rebase_path unnecessarily split: FORMAT_TEST(034)
FORMAT_TEST(035)
FORMAT_TEST(036)
FORMAT_TEST(037)
FORMAT_TEST(038)
FORMAT_TEST(039)
FORMAT_TEST(040)
FORMAT_TEST(041)
FORMAT_TEST(042)
FORMAT_TEST(043)
FORMAT_TEST(044)
FORMAT_TEST(045)
FORMAT_TEST(046)
FORMAT_TEST(047)
FORMAT_TEST(048)
// TODO(scottmg): Eval is broken (!) and comment output might have extra ,
//                FORMAT_TEST(049)
FORMAT_TEST(050)
FORMAT_TEST(051)
FORMAT_TEST(052)
FORMAT_TEST(053)
FORMAT_TEST(054)
FORMAT_TEST(055)
FORMAT_TEST(056)
FORMAT_TEST(057)
FORMAT_TEST(058)
FORMAT_TEST(059)
FORMAT_TEST(060)
FORMAT_TEST(061)
FORMAT_TEST(062)
FORMAT_TEST(063)
FORMAT_TEST(064)
FORMAT_TEST(065)
FORMAT_TEST(066)
FORMAT_TEST(067)
FORMAT_TEST(068)
FORMAT_TEST(069)
FORMAT_TEST(070)
FORMAT_TEST(071)
FORMAT_TEST(072)
FORMAT_TEST(073)
FORMAT_TEST(074)
FORMAT_TEST(075)
FORMAT_TEST(076)
FORMAT_TEST(077)
FORMAT_TEST(078)
FORMAT_TEST(079)
FORMAT_TEST(080)
FORMAT_TEST(081)
FORMAT_TEST(082)
FORMAT_TEST(083)
FORMAT_TEST(084)
FORMAT_TEST(085)
FORMAT_TEST(086)
FORMAT_TEST(087)
FORMAT_TEST(088)
FORMAT_TEST(089)
FORMAT_TEST(090)
FORMAT_TEST(091)
FORMAT_TEST(092)
FORMAT_TEST(093)
FORMAT_TEST(094)
FORMAT_TEST(095)
FORMAT_TEST(096)
FORMAT_TEST(097)
FORMAT_TEST(098)
FORMAT_TEST(099)
FORMAT_TEST(100)
FORMAT_TEST(101)
FORMAT_TEST(102)
FORMAT_TEST(103)
FORMAT_TEST(104)
FORMAT_TEST(105)
FORMAT_TEST_WITH_WIDTH(106, 20)
