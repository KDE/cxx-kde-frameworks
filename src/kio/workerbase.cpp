// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/workerbase.h"

#include <QUrl>

WorkerGlue::WorkerGlue(const QByteArray &protocol, const QByteArray &poolSocket, const QByteArray &appSocket, ::WorkerBase *worker)
    : KIO::WorkerBase(protocol, poolSocket, appSocket)
    , m_worker(worker)
{
}

KIO::WorkerResult WorkerGlue::get(const QUrl &url)
{
    return KIO::WorkerResult(*m_worker->get(url));
}

KIO::WorkerResult WorkerGlue::listDir(const QUrl &url)
{
    return KIO::WorkerResult(*m_worker->listDir(url));
}

KIO::WorkerResult WorkerGlue::stat(const QUrl &url)
{
    return KIO::WorkerResult(*m_worker->stat(url));
}

KIO::WorkerResult WorkerGlue::write(const QByteArray &data)
{
    return KIO::WorkerResult(*m_worker->write(data));
}

WorkerBase::WorkerBase(const QByteArray &protocol, const QByteArray &poolSocket, const QByteArray &appSocket)
    : m_glue(protocol, poolSocket, appSocket, this)
{
}

void WorkerBase::dispatchLoop()
{
    m_glue.dispatchLoop();
}

void WorkerBase::listEntry(const KIO::UDSEntry &entry)
{
    m_glue.listEntry(entry);
}

void WorkerBase::statEntry(const KIO::UDSEntry &entry)
{
    m_glue.statEntry(entry);
}

void WorkerBase::dataReq()
{
    m_glue.dataReq();
}

void WorkerBase::workerStatus(const QString &host, bool connected)
{
    m_glue.workerStatus(host, connected);
}

bool WorkerBase::canResume(KIO::filesize_t offset)
{
    return m_glue.canResume(offset);
}

void WorkerBase::data(const QByteArray &data)
{
    m_glue.data(data);
}

std::unique_ptr<WorkerResult> workerResultPass()
{
    return std::make_unique<KIO::WorkerResult>(KIO::WorkerResult::pass());
}

std::unique_ptr<WorkerResult> workerResultFail(int code, const QString &message)
{
    return std::make_unique<KIO::WorkerResult>(KIO::WorkerResult::fail(code, message));
}
