// RGB schemata by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fs, io};

use rgb_schemata::{nia_rgb20, nia_schema};
use rgbstd::containers::BindleContent;

fn main() -> io::Result<()> {
    nia()?;
    uda()?;

    Ok(())
}

fn nia() -> io::Result<()> {
    let schema_bindle = nia_schema().bindle();
    schema_bindle.save("schemata/NonInflatableAssets.rgb")?;
    fs::write("schemata/NonInflatableAssets.rgba", schema_bindle.to_string())?;

    let iimpl_bindle = nia_rgb20().bindle();
    iimpl_bindle.save("schemata/NonInflatableAssets-RGB20.rgb")?;
    fs::write("schemata/NonInflatableAssets-RGB20.rgba", iimpl_bindle.to_string())?;

    Ok(())
}

fn uda() -> io::Result<()> {
    let schema_bindle = nia_schema().bindle();
    schema_bindle.save("schemata/UniqueDigitalAsset.rgb")?;
    fs::write("schemata/UniqueDigitalAsset.rgba", schema_bindle.to_string())?;

    let iimpl_bindle = nia_rgb20().bindle();
    iimpl_bindle.save("schemata/UniqueDigitalAsset-RGB20.rgb")?;
    fs::write("schemata/UniqueDigitalAsset-RGB20.rgba", iimpl_bindle.to_string())?;

    Ok(())
}
