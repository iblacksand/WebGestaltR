// Generated by using Rcpp::compileAttributes() -> do not edit by hand
// Generator token: 10BE3573-1514-4C36-9D1C-5A225CD40393

#include <Rcpp.h>

using namespace Rcpp;

#ifdef RCPP_USE_GLOBAL_ROSTREAM
Rcpp::Rostream<true>&  Rcpp::Rcout = Rcpp::Rcpp_cout_get();
Rcpp::Rostream<false>& Rcpp::Rcerr = Rcpp::Rcpp_cerr_get();
#endif

// fillInputDataFrame
DataFrame fillInputDataFrame(DataFrame gmt, CharacterVector genes, CharacterVector geneSets);
RcppExport SEXP _WebGestaltR_fillInputDataFrame(SEXP gmtSEXP, SEXP genesSEXP, SEXP geneSetsSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< DataFrame >::type gmt(gmtSEXP);
    Rcpp::traits::input_parameter< CharacterVector >::type genes(genesSEXP);
    Rcpp::traits::input_parameter< CharacterVector >::type geneSets(geneSetsSEXP);
    rcpp_result_gen = Rcpp::wrap(fillInputDataFrame(gmt, genes, geneSets));
    return rcpp_result_gen;
END_RCPP
}
// gseaPermutation
NumericVector gseaPermutation(NumericMatrix inset_scores, NumericMatrix outset_scores, NumericVector expression_value);
RcppExport SEXP _WebGestaltR_gseaPermutation(SEXP inset_scoresSEXP, SEXP outset_scoresSEXP, SEXP expression_valueSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< NumericMatrix >::type inset_scores(inset_scoresSEXP);
    Rcpp::traits::input_parameter< NumericMatrix >::type outset_scores(outset_scoresSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type expression_value(expression_valueSEXP);
    rcpp_result_gen = Rcpp::wrap(gseaPermutation(inset_scores, outset_scores, expression_value));
    return rcpp_result_gen;
END_RCPP
}

RcppExport SEXP wrap__hello_world();

static const R_CallMethodDef CallEntries[] = {
    {"_WebGestaltR_fillInputDataFrame", (DL_FUNC) &_WebGestaltR_fillInputDataFrame, 3},
    {"_WebGestaltR_gseaPermutation", (DL_FUNC) &_WebGestaltR_gseaPermutation, 3},
    {"wrap__hello_world", (DL_FUNC) &wrap__hello_world, 0},
    {NULL, NULL, 0}
};

RcppExport void R_init_WebGestaltR(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
