use std::{collections::HashMap, rc::Rc};

use olympus_parser::{
	ParsedBultin, ParsedEnum, ParsedEnumVariant, ParsedProcedure, ParsedProcedureParam, ParsedStruct,
	ParsedStructField, ParsedTypeKind, Parser,
};
use olympus_spanned::{CodeSource, ErrorColor, OlympusError, Spanned};

fn find_duplicate_ident(idents: &[Spanned<String>]) -> Option<(Spanned<String>, Spanned<String>)> {
	let mut idents_map = HashMap::<String, (Spanned<String>, Option<Spanned<String>>)>::new();

	for ident in idents {
		if let Some((_, duplicated)) = idents_map.get_mut(&ident.value) {
			*duplicated = Some(ident.clone());
		} else {
			idents_map.insert(ident.value.clone(), (ident.clone(), None));
		}
	}

	for (_, (original_ident, dup_ident)) in idents_map {
		if let Some(dup_ident) = dup_ident {
			return Some((original_ident, dup_ident));
		}
	}

	None
}

fn find_enum_variant_duplicates(source: Rc<CodeSource>, variants: &[ParsedEnumVariant]) -> Result<(), OlympusError> {
	if let Some((original, dup)) = find_duplicate_ident(&variants.iter().map(|v| v.ident.clone()).collect::<Vec<_>>()) {
		return Err(OlympusError::new("Duplicate variant ident found")
			.label(source.clone(), "Original here", original.span, ErrorColor::Yellow)
			.label(source, "Duplicate here", dup.span, ErrorColor::Red));
	}

	let mut values = HashMap::<i16, (Spanned<String>, Option<Spanned<String>>)>::new();

	for variant in variants {
		if let Some((_, duplicated)) = values.get_mut(&variant.value) {
			*duplicated = Some(variant.ident.clone());
		} else {
			values.insert(variant.value, (variant.ident.clone(), None));
		}
	}

	for (_, (original_ident, dup_ident)) in values {
		if let Some(dup_ident) = dup_ident {
			return Err(OlympusError::new("Duplicate variant value found")
				.label(source.clone(), "Original here", original_ident.span, ErrorColor::Yellow)
				.label(source, "Duplicate here", dup_ident.span, ErrorColor::Red));
		}
	}

	Ok(())
}

fn find_struct_field_duplicates(source: Rc<CodeSource>, fields: &[ParsedStructField]) -> Result<(), OlympusError> {
	if let Some((original, dup)) = find_duplicate_ident(&fields.iter().map(|v| v.ident.clone()).collect::<Vec<_>>()) {
		return Err(OlympusError::new("Duplicate field ident found")
			.label(source.clone(), "Original here", original.span, ErrorColor::Yellow)
			.label(source, "Duplicate here", dup.span, ErrorColor::Red));
	}

	Ok(())
}

fn find_rpc_procedure_duplicates(source: Rc<CodeSource>, procs: &[ParsedProcedure]) -> Result<(), OlympusError> {
	let mut idents = HashMap::<String, (Spanned<String>, Option<Spanned<String>>)>::new();

	for proc in procs {
		if let Some((_, duplicated)) = idents.get_mut(&proc.ident.value) {
			*duplicated = Some(proc.ident.clone());
		} else {
			idents.insert(proc.ident.value.clone(), (proc.ident.clone(), None));
		}
	}

	for (_, (original_ident, dup_ident)) in idents {
		if let Some(dup_ident) = dup_ident {
			return Err(OlympusError::new("Duplicate proc ident found")
				.label(source.clone(), "Original here", original_ident.span, ErrorColor::Yellow)
				.label(source, "Duplicate here", dup_ident.span, ErrorColor::Red));
		}
	}

	Ok(())
}

fn find_rpc_procedure_param_duplicates(
	source: Rc<CodeSource>,
	params: &[ParsedProcedureParam],
) -> Result<(), OlympusError> {
	if let Some((original, dup)) = find_duplicate_ident(&params.iter().map(|v| v.ident.clone()).collect::<Vec<_>>()) {
		return Err(OlympusError::new("Duplicate proc param ident found")
			.label(source.clone(), "Original here", original.span, ErrorColor::Yellow)
			.label(source, "Duplicate here", dup.span, ErrorColor::Red));
	}

	Ok(())
}

fn check_accessible_type(
	source: Rc<CodeSource>,
	accessible_types: &[Spanned<String>],
	asking_for: &Spanned<ParsedTypeKind>,
) -> Result<(), OlympusError> {
	if let Spanned {
		value: ParsedTypeKind::External(external),
		span,
	} = asking_for
	{
		if !accessible_types.iter().any(|t| external == &t.value) {
			return Err(OlympusError::error(
				source,
				&format!("Type '{external}' not found"),
				span.clone(),
			));
		}
	} else if let Spanned {
		value: ParsedTypeKind::Builtin(ParsedBultin::Array(ty)),
		..
	} = asking_for
	{
		check_accessible_type(source, accessible_types, ty)?;
	}

	Ok(())
}

pub fn verify_parser_outputs(
	Parser {
		source,
		enums: parsed_enums,
		structs: parsed_structs,
		procedures,
		..
	}: &Parser,
) -> Result<(), OlympusError> {
	let accessible_types = parsed_enums
		.iter()
		.map(|v| v.ident.clone())
		.chain(parsed_structs.iter().map(|v| v.ident.clone()))
		.collect::<Vec<_>>();

	// checking for duplicate idents / values

	if let Some((original_ident, dup_ident)) = find_duplicate_ident(&accessible_types) {
		return Err(OlympusError::new("Duplicate enum/struct ident found")
			.label(source.clone(), "Original here", original_ident.span, ErrorColor::Yellow)
			.label(source.clone(), "Duplicate here", dup_ident.span, ErrorColor::Red));
	}

	for ParsedEnum { ident: _, variants } in parsed_enums {
		find_enum_variant_duplicates(source.clone(), variants)?;
	}

	for ParsedStruct {
		ident: struct_ident,
		fields,
	} in parsed_structs
	{
		find_struct_field_duplicates(source.clone(), fields)?;

		for field in fields {
			if let Spanned {
				value: ParsedTypeKind::External(external),
				span,
			} = &field.kind
			{
				if &struct_ident.value == external {
					return Err(OlympusError::error(
						source.clone(),
						"Self referencing field type",
						span.clone(),
					));
				}
			}
		}
	}

	find_rpc_procedure_duplicates(source.clone(), procedures)?;
	for proc in procedures {
		find_rpc_procedure_param_duplicates(source.clone(), &proc.params)?;
	}

	// checking that types are actually there

	for ParsedStruct { ident: _, fields } in parsed_structs {
		for field in fields {
			check_accessible_type(source.clone(), &accessible_types, &field.kind)?;
		}
	}

	for proc in procedures {
		for param in &proc.params {
			check_accessible_type(source.clone(), &accessible_types, &param.kind)?;
		}

		check_accessible_type(source.clone(), &accessible_types, &proc.return_kind)?;
	}

	Ok(())
}
