#include "cxx-kde-frameworks/ksandbox.h"

#include <QProcessEnvironment>

namespace rust {
namespace kf6 {

std::unique_ptr<KSandbox::ProcessContext> makeHostContext(const QString &workingDirectory, const QString &program,
                const QStringList &arguments, const QProcessEnvironment &env) {
    QProcess proc;
    proc.setProgram(program);
    proc.setWorkingDirectory(workingDirectory);
    proc.setArguments(arguments);
    proc.setProcessEnvironment(env);

    return std::make_unique<KSandbox::ProcessContext>(KSandbox::makeHostContext(proc));
}

QString contextGetProgram(const KSandbox::ProcessContext &ctx) {
    return ctx.program;
}

QStringList contextGetArgs(const KSandbox::ProcessContext &ctx) {
    return ctx.arguments;
}

std::unique_ptr<QProcessEnvironment> systemEnvironment() {
    return std::make_unique<QProcessEnvironment>(QProcessEnvironment::systemEnvironment());
}

} // namespace kf6
} // namespace rust
