#pragma once

#include <KSandbox>

namespace rust {
namespace kf6 {

std::unique_ptr<KSandbox::ProcessContext> makeHostContext(const QString &workingDirectory,
                    const QString &program,
                    const QStringList &arguments,
                    const QProcessEnvironment &env);

QString contextGetProgram(const KSandbox::ProcessContext &ctx);
QStringList contextGetArgs(const KSandbox::ProcessContext &ctx);

std::unique_ptr<QProcessEnvironment> systemEnvironment();

}
}
