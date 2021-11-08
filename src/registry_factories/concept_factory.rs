use std::collections::HashMap;
use legalios::service::period::IPeriod;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::concept_define::IConceptDefine;
use crate::service_types::version_code::VersionCode;
use crate::registry_constants::concept_consts::ConceptConst;
use crate::registry_providers::concept_provider::{BoxConceptSpec, BoxConceptSpecProvider, ConceptSpec, ConceptSpecProvider, IConceptSpec, IConceptSpecProvider, ResultFunc};

type MapConceptCode = i32;

pub(crate) struct NotFoundConceptSpec {
    spec: ConceptSpec,
}

impl NotFoundConceptSpec {
    fn get(_code: ConceptCode) -> NotFoundConceptSpec {
        NotFoundConceptSpec {
            spec: ConceptSpec::new(_code, vec![], None),
        }
    }
    fn new() -> NotFoundConceptSpec {
        let concept_code = ConceptConst::ConceptNotfound;
        NotFoundConceptSpec::get(ConceptCode::get(concept_code as i32))
    }
}

impl IConceptDefine for NotFoundConceptSpec {
    fn get_code(&self) -> ConceptCode {
        self.spec.get_code()
    }
}

impl IConceptSpec for NotFoundConceptSpec {
    fn get_path(&self) -> Vec<ArticleCode> {
        self.spec.get_path()
    }

    fn get_result_delegate(&self) -> Option<ResultFunc> {
        self.spec.get_result_delegate()
    }
}

pub(crate) struct NotFoundConceptProvider {
    spec: ConceptSpecProvider,
}

impl NotFoundConceptProvider {
    fn new() -> NotFoundConceptProvider {
        let concept_code = ConceptConst::ConceptNotfound;
        NotFoundConceptProvider {
            spec: ConceptSpecProvider::new(ConceptCode::get(concept_code as i32))
        }
    }
}

impl IConceptSpecProvider for NotFoundConceptProvider {
    fn get_code(&self) -> ConceptCode {
        self.spec.get_code()
    }

    fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec {
        Box::new(NotFoundConceptSpec::get(self.spec.get_code()))
    }
}

pub(crate) trait IConceptSpecFactory {
    fn get_spec(&self, code: &ConceptCode, period: &dyn IPeriod, version: &VersionCode) -> BoxConceptSpec;
    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<BoxConceptSpec>;
}

pub(crate) type BoxConceptSpecFactory = Box<dyn IConceptSpecFactory>;

pub(crate) struct ConceptSpecFactory {
    not_found_provider: BoxConceptSpecProvider,
    providers: HashMap<MapConceptCode, BoxConceptSpecProvider>,
}

impl IConceptSpecFactory for ConceptSpecFactory {
    fn get_spec(&self, code: &ConceptCode, period: &dyn IPeriod, version: &VersionCode) -> BoxConceptSpec {
        let opt_provider = self.get_provider(code, &self.not_found_provider);
        match opt_provider {
            Some(provider) => provider.get_spec(period, version),
            None => Box::new(NotFoundConceptSpec::new()),
        }
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<BoxConceptSpec> {
        self.providers.iter().map(|x| x.1.get_spec(period, version)).collect()
    }
}

type ConceptBuildFunc = fn() -> Vec<BoxConceptSpecProvider>;

impl ConceptSpecFactory {
    pub(crate) fn new(build_func: ConceptBuildFunc) -> ConceptSpecFactory {
        ConceptSpecFactory {
            not_found_provider: Box::new(NotFoundConceptProvider::new()),
            providers: build_func().into_iter().map(|x| (x.get_code().value, x)).collect()
        }
    }
    fn get_provider<'a>(&'a self, code: &ConceptCode, def_provider: &'a BoxConceptSpecProvider) -> Option<&'a BoxConceptSpecProvider> {
        let map_provider = self.providers.get(&code.value);
        let val_provider = match map_provider {
            Some(provider) => provider,
            None => def_provider,
        };
        Some(val_provider)
    }
}

