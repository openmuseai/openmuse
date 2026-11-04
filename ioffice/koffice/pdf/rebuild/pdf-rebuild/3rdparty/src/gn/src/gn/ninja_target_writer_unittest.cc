// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <sstream>

#include "gn/ninja_action_target_writer.h"
#include "gn/ninja_group_target_writer.h"
#include "gn/ninja_target_writer.h"
#include "gn/target.h"
#include "gn/test_with_scope.h"
#include "util/test/test.h"

namespace {

class TestingNinjaTargetWriter : public NinjaTargetWriter {
 public:
  TestingNinjaTargetWriter(const Target* target,
                           const Toolchain* toolchain,
                           std::ostream& out)
      : NinjaTargetWriter(target, out) {}

  void Run() override {}

  // Make this public so the test can call it.
  NinjaTargetWriter::InputDeps WriteInputDepsStampOrPhonyAndGetDep(
      const std::vector<const Target*>& additional_hard_deps,
      size_t num_stamp_uses) {
    return NinjaTargetWriter::WriteInputDepsStampOrPhonyAndGetDep(
        additional_hard_deps, num_stamp_uses);
  }

  void WriteStampOrPhonyForTarget(
      const std::vector<OutputFile>& deps,
      const std::vector<OutputFile>& order_only_deps) {
    NinjaTargetWriter::WriteStampOrPhonyForTarget(deps, order_only_deps);
  }
};

}  // namespace

TEST(NinjaTargetWriter, ResolvedCreatedOnDemand) {
  TestWithScope setup;
  Err err;

  // Make a base target that's a hard dep (action).
  Target base_target(setup.settings(), Label(SourceDir("//foo/"), "base"));
  base_target.set_output_type(Target::ACTION);
  base_target.visibility().SetPublic();
  base_target.SetToolchain(setup.toolchain());
  base_target.action_values().set_script(SourceFile("//foo/script.py"));
  ASSERT_TRUE(base_target.OnResolved(&err));

  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&base_target, setup.toolchain(), stream);

  const auto* resolved_ptr = &writer.resolved();
  ASSERT_TRUE(resolved_ptr);

  // Same address should be returned on second call.
  EXPECT_EQ(resolved_ptr, &writer.resolved());
}

TEST(NinjaTargetWriter, ResolvedSetExplicitly) {
  TestWithScope setup;
  Err err;

  // Make a base target that's a hard dep (action).
  Target base_target(setup.settings(), Label(SourceDir("//foo/"), "base"));
  base_target.set_output_type(Target::ACTION);
  base_target.visibility().SetPublic();
  base_target.SetToolchain(setup.toolchain());
  base_target.action_values().set_script(SourceFile("//foo/script.py"));
  ASSERT_TRUE(base_target.OnResolved(&err));

  ResolvedTargetData resolved;
  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&base_target, setup.toolchain(), stream);
  writer.SetResolvedTargetData(&resolved);

  EXPECT_EQ(&resolved, &writer.resolved());
}

TEST(NinjaTargetWriter, WriteInputDepsStampOrPhonyAndGetDep) {
  TestWithScope setup;
  Err err;

  // Make a base target that's a hard dep (action).
  Target base_target(setup.settings(), Label(SourceDir("//foo/"), "base"));
  base_target.set_output_type(Target::ACTION);
  base_target.visibility().SetPublic();
  base_target.SetToolchain(setup.toolchain());
  base_target.action_values().set_script(SourceFile("//foo/script.py"));

  // Dependent target that also includes a source prerequisite (should get
  // included) and a source (should not be included).
  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::EXECUTABLE);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  target.config_values().inputs().push_back(SourceFile("//foo/input.txt"));
  target.sources().push_back(SourceFile("//foo/source.txt"));
  target.public_deps().push_back(LabelTargetPair(&base_target));

  // Dependent action to test that action sources will be treated the same as
  // inputs.
  Target action(setup.settings(), Label(SourceDir("//foo/"), "action"));
  action.set_output_type(Target::ACTION);
  action.visibility().SetPublic();
  action.SetToolchain(setup.toolchain());
  action.action_values().set_script(SourceFile("//foo/script.py"));
  action.sources().push_back(SourceFile("//foo/action_source.txt"));
  action.public_deps().push_back(LabelTargetPair(&target));

  ASSERT_TRUE(base_target.OnResolved(&err));
  ASSERT_TRUE(target.OnResolved(&err));
  ASSERT_TRUE(action.OnResolved(&err));

  // Input deps for the base (should be only the script itself).
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&base_target, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    // Since there is only one dependency, it should just be returned and
    // nothing written to the stream.
    ASSERT_EQ(0u, dep.implicit.size());
    ASSERT_EQ(1u, dep.order_only.size());
    EXPECT_EQ("../../foo/script.py", dep.order_only[0].value());
    EXPECT_EQ("", stream.str());
  }

  // Input deps for the target (should depend on the base).
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    // Since there is only one dependency, a stamp file will be returned
    // directly without writing any additional rules.
    ASSERT_EQ(0u, dep.implicit.size());
    ASSERT_EQ(1u, dep.order_only.size());
    EXPECT_EQ("phony/foo/base", dep.order_only[0].value());
  }

  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&action, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_action___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:action()\n"
        "  restat = 1\n"
        "\n"
        "build: __foo_action___rule | ../../foo/script.py"
        " ../../foo/action_source.txt ./target\n"
        "\n"
        "build phony/foo/action: phony ./target\n",
        stream.str());
  }

  // Input deps for action which should depend on the base since its a hard dep
  // that is a (indirect) dependency, as well as the the action source.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&action, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    ASSERT_EQ(0u, dep.order_only.size());
    EXPECT_EQ("phony/foo/action.inputdeps", dep.implicit[0].value());
    EXPECT_EQ(
        "build phony/foo/action.inputdeps: phony ../../foo/script.py "
        "../../foo/action_source.txt ./target\n",
        stream.str());
  }
}

TEST(NinjaTargetWriter, WriteInputDepsStampOrPhonyAndGetDepUseStampFiles) {
  TestWithScope setup;
  Err err;
  setup.build_settings()->set_no_stamp_files(false);

  // Make a base target that's a hard dep (action).
  Target base_target(setup.settings(), Label(SourceDir("//foo/"), "base"));
  base_target.set_output_type(Target::ACTION);
  base_target.visibility().SetPublic();
  base_target.SetToolchain(setup.toolchain());
  base_target.action_values().set_script(SourceFile("//foo/script.py"));

  // Dependent target that also includes a source prerequisite (should get
  // included) and a source (should not be included).
  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::EXECUTABLE);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  target.config_values().inputs().push_back(SourceFile("//foo/input.txt"));
  target.sources().push_back(SourceFile("//foo/source.txt"));
  target.public_deps().push_back(LabelTargetPair(&base_target));

  // Dependent action to test that action sources will be treated the same as
  // inputs.
  Target action(setup.settings(), Label(SourceDir("//foo/"), "action"));
  action.set_output_type(Target::ACTION);
  action.visibility().SetPublic();
  action.SetToolchain(setup.toolchain());
  action.action_values().set_script(SourceFile("//foo/script.py"));
  action.sources().push_back(SourceFile("//foo/action_source.txt"));
  action.public_deps().push_back(LabelTargetPair(&target));

  ASSERT_TRUE(base_target.OnResolved(&err));
  ASSERT_TRUE(target.OnResolved(&err));
  ASSERT_TRUE(action.OnResolved(&err));

  // Input deps for the base (should be only the script itself).
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&base_target, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    // Since there is only one dependency, it should just be returned and
    // nothing written to the stream.
    ASSERT_EQ(0u, dep.implicit.size());
    ASSERT_EQ(1u, dep.order_only.size());
    EXPECT_EQ("../../foo/script.py", dep.order_only[0].value());
    EXPECT_EQ("", stream.str());
  }

  // Input deps for the target (should depend on the base).
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    // Since there is only one dependency, a stamp file will be returned
    // directly without writing any additional rules.
    ASSERT_EQ(0u, dep.implicit.size());
    ASSERT_EQ(1u, dep.order_only.size());
    EXPECT_EQ("obj/foo/base.stamp", dep.order_only[0].value());
  }

  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&action, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_action___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:action()\n"
        "  restat = 1\n"
        "\n"
        "build: __foo_action___rule | ../../foo/script.py"
        " ../../foo/action_source.txt ./target\n"
        "\n"
        "build obj/foo/action.stamp: stamp ./target\n",
        stream.str());
  }

  // Input deps for action which should depend on the base since its a hard dep
  // that is a (indirect) dependency, as well as the the action source.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&action, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    ASSERT_EQ(0u, dep.order_only.size());
    EXPECT_EQ("obj/foo/action.inputdeps.stamp", dep.implicit[0].value());
    EXPECT_EQ(
        "build obj/foo/action.inputdeps.stamp: stamp ../../foo/script.py "
        "../../foo/action_source.txt ./target\n",
        stream.str());
  }
}

// Tests WriteInputDepsStampOrPhonyAndGetDep when toolchain deps are present.
TEST(NinjaTargetWriter, WriteInputDepsStampOrPhonyAndGetDepWithToolchainDeps) {
  TestWithScope setup;
  Err err;

  // Toolchain dependency. Here we make a target in the same toolchain for
  // simplicity, but in real life (using the Builder) this would be rejected
  // because it would be a circular dependency (the target depends on its
  // toolchain, and the toolchain depends on this target).
  Target toolchain_dep_target(setup.settings(),
                              Label(SourceDir("//foo/"), "setup"));
  toolchain_dep_target.set_output_type(Target::ACTION);
  toolchain_dep_target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(toolchain_dep_target.OnResolved(&err));
  setup.toolchain()->deps().push_back(LabelTargetPair(&toolchain_dep_target));

  // Make a binary target
  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::EXECUTABLE);
  target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);
  auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
      std::vector<const Target*>(), 10u);

  // Since there is more than one dependency, a stamp file will be returned
  // and the rule for the stamp file will be written to the stream.
  ASSERT_EQ(1u, dep.implicit.size());
  ASSERT_EQ(0u, dep.order_only.size());
  EXPECT_EQ("phony/foo/setup", dep.implicit[0].value());
  EXPECT_EQ("", stream.str());
}

// Tests WriteInputDepsStampOrPhonyAndGetDep when a toolchain dependency and
// another dependency are present.
TEST(NinjaTargetWriter,
     WriteInputDepsStampOrPhonyAndGetDepWithToolchainAndAnotherDep) {
  TestWithScope setup;
  Err err;

  Target toolchain_dep_target(setup.settings(),
                              Label(SourceDir("//foo/"), "setup"));
  toolchain_dep_target.set_output_type(Target::ACTION);
  toolchain_dep_target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(toolchain_dep_target.OnResolved(&err));
  setup.toolchain()->deps().push_back(LabelTargetPair(&toolchain_dep_target));

  Target base_target(setup.settings(), Label(SourceDir("//foo/"), "base"));
  base_target.set_output_type(Target::ACTION);
  base_target.visibility().SetPublic();
  base_target.SetToolchain(setup.toolchain());
  base_target.action_values().set_script(SourceFile("//foo/script.py"));
  ASSERT_TRUE(base_target.OnResolved(&err));

  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::EXECUTABLE);
  target.SetToolchain(setup.toolchain());
  target.public_deps().push_back(LabelTargetPair(&base_target));
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);
  auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
      std::vector<const Target*>(), 10u);

  // Since there is a toolchain dependency, it should be returned as an implicit
  // dependency.
  ASSERT_EQ(1u, dep.implicit.size());
  EXPECT_EQ("phony/foo/target.inputdeps", dep.implicit[0].value());
  ASSERT_EQ(0u, dep.order_only.size());
  EXPECT_EQ(
      "build phony/foo/target.inputdeps: phony phony/foo/setup || "
      "phony/foo/base\n",
      stream.str());
}

// Tests that validation dependencies are written to the generated Ninja file
// with the "|@" syntax for a generic target.
TEST(NinjaTargetWriter, WriteValidations) {
  TestWithScope setup;
  setup.build_settings()->set_no_stamp_files(false);
  Err err;

  Target validation_target(setup.settings(), Label(SourceDir("//foo/"), "val"));
  validation_target.set_output_type(Target::ACTION);
  validation_target.visibility().SetPublic();
  validation_target.SetToolchain(setup.toolchain());
  validation_target.action_values().set_script(SourceFile("//foo/script.py"));
  ASSERT_TRUE(validation_target.OnResolved(&err));

  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::GROUP);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  target.validations().push_back(LabelTargetPair(&validation_target));
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);

  std::vector<OutputFile> deps;
  std::vector<OutputFile> order_only;
  deps.push_back(OutputFile("obj/foo/target.stamp"));

  writer.WriteStampOrPhonyForTarget(deps, order_only);

  std::string out = stream.str();
  EXPECT_EQ(
      "build obj/foo/target.stamp: stamp obj/foo/target.stamp |@ "
      "obj/foo/val.stamp\n",
      out);
}

// Tests that if a validation target has no output (e.g., an empty group),
// no validation dependency is written to the Ninja file.
TEST(NinjaTargetWriter, ValidationsWithNoOutput) {
  TestWithScope setup;
  setup.build_settings()->set_no_stamp_files(true);
  Err err;

  // Validation target with no output (empty group, no stamp files).
  Target validation_target(setup.settings(), Label(SourceDir("//foo/"), "val"));
  validation_target.set_output_type(Target::GROUP);
  validation_target.visibility().SetPublic();
  validation_target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(validation_target.OnResolved(&err));
  ASSERT_FALSE(validation_target.has_dependency_output());

  // A dependency that HAS an output, so 'target' will have an output alias.
  Target real_dep(setup.settings(), Label(SourceDir("//foo/"), "dep"));
  real_dep.set_output_type(Target::ACTION);
  real_dep.visibility().SetPublic();
  real_dep.SetToolchain(setup.toolchain());
  real_dep.action_values().set_script(SourceFile("//foo/script.py"));
  ASSERT_TRUE(real_dep.OnResolved(&err));

  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::GROUP);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  target.public_deps().push_back(LabelTargetPair(&real_dep));
  target.validations().push_back(LabelTargetPair(&validation_target));
  ASSERT_TRUE(target.OnResolved(&err));
  ASSERT_TRUE(target.has_dependency_output());

  std::ostringstream stream;
  TestingNinjaTargetWriter writer(&target, setup.toolchain(), stream);

  std::vector<OutputFile> deps;
  std::vector<OutputFile> order_only;
  deps.push_back(OutputFile("phony/foo/dep"));

  writer.WriteStampOrPhonyForTarget(deps, order_only);

  std::string out = stream.str();
  // Should not contain validation separator since the validation target has no
  // output.
  EXPECT_EQ("build phony/foo/target: phony phony/foo/dep\n", out);
}

TEST(NinjaTargetWriter, PhonyPropagation) {
  TestWithScope setup;
  Err err;

  // Set up common target A (action)
  Target a(setup.settings(), Label(SourceDir("//foo/"), "a"));
  a.set_output_type(Target::ACTION);
  a.visibility().SetPublic();
  a.SetToolchain(setup.toolchain());
  a.action_values().set_script(SourceFile("//foo/script.py"));
  a.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/a.out");

  // Set up B_pub (action, public_dep A)
  Target b_pub(setup.settings(), Label(SourceDir("//foo/"), "b_pub"));
  b_pub.set_output_type(Target::ACTION);
  b_pub.visibility().SetPublic();
  b_pub.SetToolchain(setup.toolchain());
  b_pub.action_values().set_script(SourceFile("//foo/script.py"));
  b_pub.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/b_pub.out");
  b_pub.public_deps().push_back(LabelTargetPair(&a));

  // Set up B_priv (action, private_dep A)
  Target b_priv(setup.settings(), Label(SourceDir("//foo/"), "b_priv"));
  b_priv.set_output_type(Target::ACTION);
  b_priv.visibility().SetPublic();
  b_priv.SetToolchain(setup.toolchain());
  b_priv.action_values().set_script(SourceFile("//foo/script.py"));
  b_priv.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/b_priv.out");
  b_priv.private_deps().push_back(LabelTargetPair(&a));

  // Set up C_pub (action, private_dep B_pub)
  Target c_pub(setup.settings(), Label(SourceDir("//foo/"), "c_pub"));
  c_pub.set_output_type(Target::ACTION);
  c_pub.visibility().SetPublic();
  c_pub.SetToolchain(setup.toolchain());
  c_pub.action_values().set_script(SourceFile("//foo/script.py"));
  c_pub.private_deps().push_back(LabelTargetPair(&b_pub));

  // Set up C_priv (action, private_dep B_priv)
  Target c_priv(setup.settings(), Label(SourceDir("//foo/"), "c_priv"));
  c_priv.set_output_type(Target::ACTION);
  c_priv.visibility().SetPublic();
  c_priv.SetToolchain(setup.toolchain());
  c_priv.action_values().set_script(SourceFile("//foo/script.py"));
  c_priv.private_deps().push_back(LabelTargetPair(&b_priv));

  // Set up B_data (action, data_dep A)
  Target b_data(setup.settings(), Label(SourceDir("//foo/"), "b_data"));
  b_data.set_output_type(Target::ACTION);
  b_data.visibility().SetPublic();
  b_data.SetToolchain(setup.toolchain());
  b_data.action_values().set_script(SourceFile("//foo/script.py"));
  b_data.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/b_data.out");
  b_data.data_deps().push_back(LabelTargetPair(&a));

  // Set up C_data (action, public_dep B_data)
  Target c_data(setup.settings(), Label(SourceDir("//foo/"), "c_data"));
  c_data.set_output_type(Target::ACTION);
  c_data.visibility().SetPublic();
  c_data.SetToolchain(setup.toolchain());
  c_data.action_values().set_script(SourceFile("//foo/script.py"));
  c_data.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/c_data.out");
  c_data.public_deps().push_back(LabelTargetPair(&b_data));

  // Set up D_data (action, private_dep C_data)
  Target d_data(setup.settings(), Label(SourceDir("//foo/"), "d_data"));
  d_data.set_output_type(Target::ACTION);
  d_data.visibility().SetPublic();
  d_data.SetToolchain(setup.toolchain());
  d_data.action_values().set_script(SourceFile("//foo/script.py"));
  d_data.private_deps().push_back(LabelTargetPair(&c_data));

  ASSERT_TRUE(a.OnResolved(&err));
  ASSERT_TRUE(b_pub.OnResolved(&err));
  ASSERT_TRUE(b_priv.OnResolved(&err));
  ASSERT_TRUE(c_pub.OnResolved(&err));
  ASSERT_TRUE(c_priv.OnResolved(&err));
  ASSERT_TRUE(b_data.OnResolved(&err));
  ASSERT_TRUE(c_data.OnResolved(&err));
  ASSERT_TRUE(d_data.OnResolved(&err));

  // 1. Verify B_pub's stamp/phony target. It should contain A's output
  // (phony/foo/a).
  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&b_pub, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_b_pub___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:b_pub()\n"
        "  restat = 1\n"
        "\n"
        "build b_pub.out: __foo_b_pub___rule | ../../foo/script.py "
        "phony/foo/a\n"
        "\n"
        "build phony/foo/b_pub: phony b_pub.out phony/foo/a\n",
        stream.str());
  }

  // 2. Verify B_priv's stamp/phony target. It should NOT contain A's output.
  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&b_priv, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_b_priv___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:b_priv()\n"
        "  restat = 1\n"
        "\n"
        "build b_priv.out: __foo_b_priv___rule | ../../foo/script.py "
        "phony/foo/a\n"
        "\n"
        "build phony/foo/b_priv: phony b_priv.out\n",
        stream.str());
  }

  // 3. Verify C_pub's inputdeps contains B_pub's stamp/phony.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&c_pub, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    EXPECT_EQ("phony/foo/c_pub.inputdeps", dep.implicit[0].value());
    EXPECT_EQ(
        "build phony/foo/c_pub.inputdeps: phony ../../foo/script.py "
        "phony/foo/b_pub\n",
        stream.str());
  }

  // 4. Verify C_priv's inputdeps contains B_priv's stamp/phony.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&c_priv, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    EXPECT_EQ("phony/foo/c_priv.inputdeps", dep.implicit[0].value());
    EXPECT_EQ(
        "build phony/foo/c_priv.inputdeps: phony ../../foo/script.py "
        "phony/foo/b_priv\n",
        stream.str());
  }

  // 5. Verify B_data's stamp/phony target. It should NOT contain A's output
  // in implicit deps (since A is a data_dep), but in order_only_deps.
  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&b_data, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_b_data___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:b_data()\n"
        "  restat = 1\n"
        "\n"
        "build b_data.out: __foo_b_data___rule | ../../foo/script.py || "
        "phony/foo/a\n"
        "\n"
        "build phony/foo/b_data: phony b_data.out\n",
        stream.str());
  }

  // 6. Verify C_data's stamp/phony target. It should contain B_data, but NOT A.
  {
    std::ostringstream stream;
    NinjaActionTargetWriter writer(&c_data, stream);
    writer.Run();
    EXPECT_EQ(
        "rule __foo_c_data___rule\n"
        "  command =  ../../foo/script.py\n"
        "  description = ACTION //foo:c_data()\n"
        "  restat = 1\n"
        "\n"
        "build c_data.out: __foo_c_data___rule | ../../foo/script.py "
        "phony/foo/b_data\n"
        "\n"
        "build phony/foo/c_data: phony c_data.out phony/foo/b_data\n",
        stream.str());
  }

  // 7. Verify D_data's inputdeps contains C_data, but NOT A.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&d_data, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    EXPECT_EQ("phony/foo/d_data.inputdeps", dep.implicit[0].value());
    EXPECT_EQ(
        "build phony/foo/d_data.inputdeps: phony ../../foo/script.py "
        "phony/foo/c_data\n",
        stream.str());
  }
}

TEST(NinjaTargetWriter, PublicInputs) {
  TestWithScope setup;
  Err err;

  // Action A has public_inputs.
  Target a(setup.settings(), Label(SourceDir("//foo/"), "a"));
  a.set_output_type(Target::ACTION);
  a.visibility().SetPublic();
  a.SetToolchain(setup.toolchain());
  a.action_values().set_script(SourceFile("//foo/script.py"));
  a.public_inputs().push_back(SourceFile("//foo/a.in"));

  // Action B depends on A.
  Target b(setup.settings(), Label(SourceDir("//foo/"), "b"));
  b.set_output_type(Target::ACTION);
  b.visibility().SetPublic();
  b.SetToolchain(setup.toolchain());
  b.action_values().set_script(SourceFile("//foo/script.py"));
  b.private_deps().push_back(LabelTargetPair(&a));

  ASSERT_TRUE(a.OnResolved(&err));
  ASSERT_TRUE(b.OnResolved(&err));

  // 1. Verify A's public_inputs phony/stamp target is written.
  {
    std::ostringstream stream;
    ResolvedTargetData resolved;
    TestingNinjaTargetWriter::WritePublicInputsStampOrPhony(&a, &resolved,
                                                            stream);
    EXPECT_EQ("build phony/foo/a.public_inputs: phony ../../foo/a.in\n\n",
              stream.str());
  }

  // 2. Verify B's input deps. It should depend on A's public_inputs.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&b, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    EXPECT_EQ("phony/foo/b.inputdeps", dep.implicit[0].value());

    std::string out = stream.str();
    EXPECT_TRUE(out.find("phony/foo/a.public_inputs") != std::string::npos)
        << out;
    EXPECT_TRUE(out.find("../../foo/a.in") == std::string::npos) << out;
  }

  // 3. Verify A's own action build definition includes its public_inputs.
  {
    std::ostringstream stream;
    // B's target needs output defined, let's configure A as well
    a.action_values().outputs() =
        SubstitutionList::MakeForTest("//out/Debug/a.out");
    NinjaActionTargetWriter writer(&a, stream);
    writer.Run();
    std::string out = stream.str();
    // out should contain "phony/foo/a.public_inputs" as an input dependency
    // instead of "../../foo/a.in".
    EXPECT_TRUE(out.contains("phony/foo/a.public_inputs")) << out;
    EXPECT_FALSE(out.contains("../../foo/a.in")) << out;
  }
}

TEST(NinjaTargetWriter, GroupPublicInputs) {
  TestWithScope setup;
  Err err;

  // Group G has public_inputs.
  Target g(setup.settings(), Label(SourceDir("//foo/"), "g"));
  g.set_output_type(Target::GROUP);
  g.visibility().SetPublic();
  g.SetToolchain(setup.toolchain());
  g.public_inputs().push_back(SourceFile("//foo/g.in"));

  // Action B depends on G.
  Target b(setup.settings(), Label(SourceDir("//foo/"), "b"));
  b.set_output_type(Target::ACTION);
  b.visibility().SetPublic();
  b.SetToolchain(setup.toolchain());
  b.action_values().set_script(SourceFile("//foo/script.py"));
  b.private_deps().push_back(LabelTargetPair(&g));

  ASSERT_TRUE(g.OnResolved(&err));
  ASSERT_TRUE(b.OnResolved(&err));

  // 1. Verify G's public_inputs phony/stamp target is written.
  {
    std::ostringstream stream;
    ResolvedTargetData resolved;
    TestingNinjaTargetWriter::WritePublicInputsStampOrPhony(&g, &resolved,
                                                            stream);
    EXPECT_EQ("build phony/foo/g.public_inputs: phony ../../foo/g.in\n\n",
              stream.str());
  }

  // 2. Verify B's input deps. It should depend on G's public_inputs.
  {
    std::ostringstream stream;
    TestingNinjaTargetWriter writer(&b, setup.toolchain(), stream);
    auto dep = writer.WriteInputDepsStampOrPhonyAndGetDep(
        std::vector<const Target*>(), 10u);

    ASSERT_EQ(1u, dep.implicit.size());
    EXPECT_EQ("phony/foo/b.inputdeps", dep.implicit[0].value());

    std::string out = stream.str();
    EXPECT_TRUE(out.contains("phony/foo/g.public_inputs")) << out;
    EXPECT_FALSE(out.contains("../../foo/g.in")) << out;
  }
}
