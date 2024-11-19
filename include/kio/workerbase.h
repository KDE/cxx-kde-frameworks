// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KIO/WorkerBase>
#include <KIO/UDSEntry>

#include "rust/cxx.h"

using WorkerResult = KIO::WorkerResult;

class WorkerBase;

class WorkerGlue : public KIO::WorkerBase
{
public:
    WorkerGlue(const QByteArray &protocol, const QByteArray &poolSocket, const QByteArray &appSocket, ::WorkerBase *worker);

    KIO::WorkerResult get(const QUrl &url) override;
    KIO::WorkerResult listDir(const QUrl &url) override;
    WorkerResult stat(const QUrl &url) override;
    WorkerResult write(const QByteArray &data) override;

private:
    ::WorkerBase *m_worker = nullptr;
};

class WorkerBase
{
public:
    WorkerBase(const QByteArray &protocol, const QByteArray &poolSocket, const QByteArray &appSocket);
    virtual ~WorkerBase() = default;

    virtual std::unique_ptr<WorkerResult> get(const QUrl &url) = 0;

    virtual std::unique_ptr<WorkerResult> listDir(const QUrl &url) = 0;

    virtual std::unique_ptr<WorkerResult> stat(const QUrl &url) = 0;

    virtual std::unique_ptr<WorkerResult> write(const QByteArray &data) = 0;

    void dispatchLoop();
    void listEntry(const KIO::UDSEntry &entry);
    void statEntry(const KIO::UDSEntry &entry);
    void dataReq();
    void workerStatus(const QString &host, bool connected);
    bool canResume(KIO::filesize_t offset);

    void data(const QByteArray &data);

private:
    WorkerGlue m_glue;
};

std::unique_ptr<WorkerResult> workerResultPass();
std::unique_ptr<WorkerResult> workerResultFail(int code, const QString &message);
