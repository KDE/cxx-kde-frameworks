#pragma once

#include <KAboutData>


#include "rust/cxx.h"

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license) -> std::unique_ptr<KAboutData>;

void setApplicationData(const KAboutData &aboutData);

KAboutPerson kaboutperson_init_default();

KAboutPerson kaboutperson_init(const QString &name, const QString &task, const QString &emailAddress, const QString &websiteAddress, const QUrl &avatarUrl);

} // namespace kf6
} // namespace rust

namespace rust {

template<>
struct IsRelocatable<KAboutPerson> : ::std::true_type
{};

}
