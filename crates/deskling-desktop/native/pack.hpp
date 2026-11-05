// SPDX-License-Identifier: MIT
#pragma once
#include <QString>
#include <QVariantMap>

// On failure pack is empty and error describes the invalid field or asset.
// An empty path selects the engine's artwork-free diagnostic shape.
bool loadCharacterPack(const QString &manifestPath, QVariantMap &pack, QString &error);
