// /-- THIS FILE WAS AUTOMATICALLY GENERATED BY OLYMPUS --\
#![allow(unused_qualifications)]
#![allow(non_snake_case)]
#[::olympus_net_common::async_trait]
pub trait ServerRpc<Ctx: Clone + Send + Sync + 'static> {
	async fn GetServerVersion(context: Ctx) -> ::olympus_net_common::Result<i8>;
	async fn GetFile(context: Ctx, params: GetFileParams) -> ::olympus_net_common::Result<File>;
	async fn DeleteFile(context: Ctx, params: DeleteFileParams) -> ::olympus_net_common::Result<()>;
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Action {
	Delete = 1,
	SecureDelete = 2,
	Encrypt = 3,
}

impl ::olympus_net_common::ProcedureInput for Action {
	fn deserialize(input: &mut ::olympus_net_common::bytes::BytesMut) -> ::olympus_net_common::Result<Self> {
		use ::olympus_net_common::bytes::Buf;
		let tag = input.get_u16();
		match tag {
			1 => Ok(Self::Delete),
			2 => Ok(Self::SecureDelete),
			3 => Ok(Self::Encrypt),
			_ => Err(::olympus_net_common::error!("invalid tag: {tag}")),
		}
	}
}

impl ::olympus_net_common::ProcedureOutput for Action {
	fn serialize(&self) -> ::olympus_net_common::Result<::olympus_net_common::bytes::BytesMut> {
		use ::olympus_net_common::bytes::BufMut;
		let mut out = ::olympus_net_common::bytes::BytesMut::with_capacity(::std::mem::size_of::<u16>());
		out.put_u16(*self as _);
		Ok(out)
	}
}

#[derive(Debug, Clone)]
pub struct File {
	pub path: String,
	pub size: ::olympus_net_common::Variable<u64>,
	pub content: Vec<u8>,
}

impl ::olympus_net_common::ProcedureInput for File {
	fn deserialize(input: &mut ::olympus_net_common::bytes::BytesMut) -> ::olympus_net_common::Result<Self> {
		Ok(Self {
			path: ::olympus_net_common::ProcedureInput::deserialize(input)?,
			size: ::olympus_net_common::ProcedureInput::deserialize(input)?,
			content: ::olympus_net_common::ProcedureInput::deserialize(input)?,
		})
	}
}

impl ::olympus_net_common::ProcedureOutput for File {
	fn serialize(&self) -> ::olympus_net_common::Result<::olympus_net_common::bytes::BytesMut> {
		let mut out = ::olympus_net_common::bytes::BytesMut::new();
		out.extend(self.path.serialize()?);
		out.extend(self.size.serialize()?);
		out.extend(self.content.serialize()?);
		Ok(out)
	}
}

#[derive(Debug, Clone)]
pub struct GetFileParams {
	pub path: String,
	pub after_action: Option<Action>,
}

impl ::olympus_net_common::ProcedureInput for GetFileParams {
	fn deserialize(input: &mut ::olympus_net_common::bytes::BytesMut) -> ::olympus_net_common::Result<Self> {
		Ok(Self {
			path: ::olympus_net_common::ProcedureInput::deserialize(input)?,
			after_action: ::olympus_net_common::ProcedureInput::deserialize(input)?,
		})
	}
}

impl ::olympus_net_common::ProcedureOutput for GetFileParams {
	fn serialize(&self) -> ::olympus_net_common::Result<::olympus_net_common::bytes::BytesMut> {
		let mut out = ::olympus_net_common::bytes::BytesMut::new();
		out.extend(self.path.serialize()?);
		out.extend(self.after_action.serialize()?);
		Ok(out)
	}
}

#[derive(Debug, Clone)]
pub struct DeleteFileParams {
	pub path: String,
}

impl ::olympus_net_common::ProcedureInput for DeleteFileParams {
	fn deserialize(input: &mut ::olympus_net_common::bytes::BytesMut) -> ::olympus_net_common::Result<Self> {
		Ok(Self {
			path: ::olympus_net_common::ProcedureInput::deserialize(input)?,
		})
	}
}

impl ::olympus_net_common::ProcedureOutput for DeleteFileParams {
	fn serialize(&self) -> ::olympus_net_common::Result<::olympus_net_common::bytes::BytesMut> {
		let mut out = ::olympus_net_common::bytes::BytesMut::new();
		out.extend(self.path.serialize()?);
		Ok(out)
	}
}
