# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache Software License 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
# which is available at https://opensource.org/licenses/MIT.
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

import pytest

import iceoryx2 as iox2


def test_service_name_can_be_constructed() -> None:
    sut = iox2.ServiceName.new("hello world")
    assert sut.to_string() == "hello world"


def test_service_name_construction_fails_when_max_len_is_exceeded() -> None:
    sut_value = "x" * (iox2.ServiceName.max_len() + 1)
    with pytest.raises(iox2.SemanticStringError):
        iox2.ServiceName.new(sut_value)
