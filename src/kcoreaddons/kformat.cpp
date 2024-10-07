#include "cxx-kde-frameworks/kformat.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(KFormat, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KFormat>::value);
static_assert(!::std::is_trivially_copy_constructible<KFormat>::value);

static_assert(!::std::is_trivially_destructible<KFormat>::value);

static_assert(QTypeInfo<KFormat>::isRelocatable);
