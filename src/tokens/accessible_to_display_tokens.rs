use std::collections::HashMap;

pub fn get_conversions() -> HashMap<String, String> {
    let conversions: HashMap<String, String> = [
        ("L1".to_string(), "L₁".to_string()),
        ("L2".to_string(), "L₂".to_string()),
        ("L3".to_string(), "L₃".to_string()),
        ("L4".to_string(), "L₄".to_string()),
        ("L5".to_string(), "L₅".to_string()),
        ("L6".to_string(), "L₆".to_string()),
        ("{Y1}".to_string(), "Y₁".to_string()),
        ("{Y2}".to_string(), "Y₂".to_string()),
        ("{Y3}".to_string(), "Y₃".to_string()),
        ("{Y4}".to_string(), "Y₄".to_string()),
        ("{Y5}".to_string(), "Y₅".to_string()),
        ("{Y6}".to_string(), "Y₆".to_string()),
        ("{Y7}".to_string(), "Y₇".to_string()),
        ("{Y8}".to_string(), "Y₈".to_string()),
        ("{Y9}".to_string(), "Y₉".to_string()),
        ("{Y0}".to_string(), "Y₀".to_string()),
        ("{X1T}".to_string(), "X₁ᴛ".to_string()),
        ("{Y1T}".to_string(), "Y₁ᴛ".to_string()),
        ("{X2T}".to_string(), "X₂ᴛ".to_string()),
        ("{Y2T}".to_string(), "Y₂ᴛ".to_string()),
        ("{X3T}".to_string(), "X₃ᴛ".to_string()),
        ("{Y3T}".to_string(), "Y₃ᴛ".to_string()),
        ("{X4T}".to_string(), "X₄ᴛ".to_string()),
        ("{Y4T}".to_string(), "Y₄ᴛ".to_string()),
        ("{X5T}".to_string(), "X₅ᴛ".to_string()),
        ("{Y5T}".to_string(), "Y₅ᴛ".to_string()),
        ("{X6T}".to_string(), "X₆ᴛ".to_string()),
        ("{Y6T}".to_string(), "Y₆ᴛ".to_string()),
        ("{r1}".to_string(), "r₁".to_string()),
        ("{r2}".to_string(), "r₂".to_string()),
        ("{r3}".to_string(), "r₃".to_string()),
        ("{r4}".to_string(), "r₄".to_string()),
        ("{r5}".to_string(), "r₅".to_string()),
        ("{r6}".to_string(), "r₆".to_string()),
        ("|u".to_string(), "u".to_string()),
        ("|v".to_string(), "v".to_string()),
        ("|w".to_string(), "w".to_string()),
        ("[RegEQ]".to_string(), "RegEQ".to_string()),
        ("[n]".to_string(), "n".to_string()),
        ("[xhat]".to_string(), "x̄".to_string()),
        ("[Sigmax]".to_string(), "Σx".to_string()),
        ("[Sigmax^2]".to_string(), "Σx²".to_string()),
        ("[Sx]".to_string(), "Sx".to_string()),
        ("[sigmax]".to_string(), "σx".to_string()),
        ("[minX]".to_string(), "minX".to_string()),
        ("[maxX]".to_string(), "maxX".to_string()),
        ("[minY]".to_string(), "minY".to_string()),
        ("[maxY]".to_string(), "maxY".to_string()),
        ("[yhat]".to_string(), "ȳ".to_string()),
        ("[Sigmay]".to_string(), "Σy".to_string()),
        ("[Sigmay^2]".to_string(), "Σy²".to_string()),
        ("[Sy]".to_string(), "Sy".to_string()),
        ("[sigmay]".to_string(), "σy".to_string()),
        ("[Sigmaxy]".to_string(), "Σxy".to_string()),
        ("[r]".to_string(), "r".to_string()),
        ("[Med]".to_string(), "Med".to_string()),
        ("[Q1]".to_string(), "Q₁".to_string()),
        ("[Q3]".to_string(), "Q₃".to_string()),
        ("[|a]".to_string(), "a".to_string()),
        ("[|b]".to_string(), "b".to_string()),
        ("[|c]".to_string(), "c".to_string()),
        ("[|d]".to_string(), "d".to_string()),
        ("[|e]".to_string(), "e".to_string()),
        ("[x1]".to_string(), "x₁".to_string()),
        ("[x2]".to_string(), "x₂".to_string()),
        ("[x3]".to_string(), "x₃".to_string()),
        ("[y1]".to_string(), "y₁".to_string()),
        ("[y2]".to_string(), "y₂".to_string()),
        ("[y3]".to_string(), "y₃".to_string()),
        ("[recursiven]".to_string(), "𝑛".to_string()),
        ("[p]".to_string(), "p".to_string()),
        ("[z]".to_string(), "z".to_string()),
        ("[t]".to_string(), "t".to_string()),
        ("[chi^2]".to_string(), "χ²".to_string()),
        ("[|F]".to_string(), "𝙵".to_string()),
        ("[df]".to_string(), "df".to_string()),
        ("[phat]".to_string(), "p̂".to_string()),
        ("[phat1]".to_string(), "p̂₁".to_string()),
        ("[phat2]".to_string(), "p̂₂".to_string()),
        ("[xhat1]".to_string(), "x̄₁".to_string()),
        ("[Sx1]".to_string(), "Sx₁".to_string()),
        ("[n1]".to_string(), "n₁".to_string()),
        ("[xhat2]".to_string(), "x̄₂".to_string()),
        ("[Sx2]".to_string(), "Sx₂".to_string()),
        ("[n2]".to_string(), "n₂".to_string()),
        ("[Sxp]".to_string(), "Sxp".to_string()),
        ("[lower]".to_string(), "lower".to_string()),
        ("[upper]".to_string(), "upper".to_string()),
        ("[s]".to_string(), "s".to_string()),
        ("[r^2]".to_string(), "r²".to_string()),
        ("[R^2]".to_string(), "R²".to_string()),
        ("[factordf]".to_string(), "df".to_string()),
        ("[factorSS]".to_string(), "SS".to_string()),
        ("[factorMS]".to_string(), "MS".to_string()),
        ("[errordf]".to_string(), "df".to_string()),
        ("[errorSS]".to_string(), "SS".to_string()),
        ("[errorMS]".to_string(), "MS".to_string()),
        ("u(nMin)".to_string(), "u(𝑛Min)".to_string()),
        ("v(nMin)".to_string(), "v(𝑛Min)".to_string()),
        ("Un-1".to_string(), "U𝑛-₁".to_string()),
        ("Vn-1".to_string(), "V𝑛-₁".to_string()),
        ("Zu(nMin)".to_string(), "Zu(𝑛Min)".to_string()),
        ("Zv(nMin)".to_string(), "Zv(𝑛Min)".to_string()),
        ("thetaMin".to_string(), "θmin".to_string()),
        ("thetaMax".to_string(), "θmax".to_string()),
        ("Zthetamin".to_string(), "Zθmin".to_string()),
        ("Zthetamax".to_string(), "Zθmax".to_string()),
        ("nMax".to_string(), "𝑛Max".to_string()),
        ("ZnMax".to_string(), "Z𝑛Max".to_string()),
        ("nMin".to_string(), "𝑛Min".to_string()),
        ("ZnMin".to_string(), "Z𝑛Min".to_string()),
        ("DeltaTbl".to_string(), "ΔTbl".to_string()),
        ("thetastep".to_string(), "θstep".to_string()),
        ("Zthetastep".to_string(), "Zθstep".to_string()),
        ("DeltaX".to_string(), "ΔX".to_string()),
        ("DeltaY".to_string(), "ΔY".to_string()),
        ("|N".to_string(), "𝗡".to_string()),
        ("|P/Y".to_string(), "P/Y".to_string()),
        ("|C/Y".to_string(), "C/Y".to_string()),
        ("w(nMin)".to_string(), "w(𝑛Min)".to_string()),
        ("Zw(nMin)".to_string(), "Zw(𝑛Min)".to_string()),
        ("SigmaPrn(".to_string(), "ΣPrn(".to_string()),
        ("SigmaInt(".to_string(), "ΣInt(".to_string()),
        (">Nom(".to_string(), "►Nom(".to_string()),
        (">Eff(".to_string(), "►Eff(".to_string()),
        ("chi^2cdf(".to_string(), "χ²cdf(".to_string()),
        ("Fcdf(".to_string(), "𝙵cdf(".to_string()),
        ("chi^2pdf(".to_string(), "χ²pdf(".to_string()),
        ("Fpdf(".to_string(), "𝙵pdf(".to_string()),
        ("tvm_N".to_string(), "tvm_𝗡".to_string()),
        ("DeltaList(".to_string(), "ΔList(".to_string()),
        (">Rect".to_string(), "►Rect".to_string()),
        (">Polar".to_string(), "►Polar".to_string()),
        ("[e]".to_string(), "𝑒".to_string()),
        ("Shadechi^2(".to_string(), "Shadeχ²(".to_string()),
        ("ShadeF(".to_string(), "Shade𝙵(".to_string()),
        ("Matr>list(".to_string(), "Matr►list(".to_string()),
        ("List>matr(".to_string(), "List►matr(".to_string()),
        ("chi^2-Test(".to_string(), "χ²-Test(".to_string()),
        ("2-SampFTest ".to_string(), "2-Samp𝙵Test ".to_string()),
        ("re^thetai".to_string(), "r𝑒^θ𝑖".to_string()),
        ("a+bi".to_string(), "a+b𝑖".to_string()),
        ("Equ>String(".to_string(), "Equ►String(".to_string()),
        ("String>Equ(".to_string(), "String►Equ(".to_string()),
        ("|'".to_string(), "´".to_string()),
        ("|`".to_string(), "`".to_string()),
        ("|:".to_string(), "¨".to_string()),
        ("|?".to_string(), "¿".to_string()),
        ("|!".to_string(), "¡".to_string()),
        ("alpha".to_string(), "α".to_string()),
        ("beta".to_string(), "β".to_string()),
        ("gamma".to_string(), "γ".to_string()),
        ("Delta".to_string(), "Δ".to_string()),
        ("delta".to_string(), "δ".to_string()),
        ("epsilon".to_string(), "ε".to_string()),
        ("lambda".to_string(), "λ".to_string()),
        ("mu".to_string(), "μ".to_string()),
        ("greek_pi".to_string(), "π".to_string()),
        ("rho".to_string(), "ρ".to_string()),
        ("Sigma".to_string(), "Σ".to_string()),
        ("Phi".to_string(), "Φ".to_string()),
        ("Omega".to_string(), "Ω".to_string()),
        ("phat".to_string(), "p̂".to_string()),
        ("chi".to_string(), "χ".to_string()),
        ("|F".to_string(), "𝙵".to_string()),
        ("sigma".to_string(), "σ".to_string()),
        ("tau".to_string(), "τ".to_string()),
        ("|~".to_string(), "~".to_string()),
        ("...".to_string(), "…".to_string()),
        ("|<".to_string(), "∠".to_string()),
        ("sharps".to_string(), "ß".to_string()),
        ("^^x".to_string(), "ˣ".to_string()),
        ("smallT".to_string(), "ᴛ".to_string()),
        ("small0".to_string(), "₀".to_string()),
        ("small1".to_string(), "₁".to_string()),
        ("small2".to_string(), "₂".to_string()),
        ("small3".to_string(), "₃".to_string()),
        ("small4".to_string(), "₄".to_string()),
        ("small5".to_string(), "₅".to_string()),
        ("small6".to_string(), "₆".to_string()),
        ("small7".to_string(), "₇".to_string()),
        ("small8".to_string(), "₈".to_string()),
        ("small9".to_string(), "₉".to_string()),
        ("small10".to_string(), "₁₀".to_string()),
        ("<|".to_string(), "◄".to_string()),
        ("|>".to_string(), "►".to_string()),
        ("uparrow".to_string(), "↑".to_string()),
        ("downarrow".to_string(), "↓".to_string()),
        ("xmark".to_string(), "×".to_string()),
        ("integral".to_string(), "∫".to_string()),
        ("bolduparrow".to_string(), "🡁".to_string()),
        ("bolddownarrow".to_string(), "🠿".to_string()),
        ("squareroot".to_string(), "√".to_string()),
        ("invertedequal".to_string(), "⌸".to_string()),
        ("chi^2GOF-Test(".to_string(), "χ²GOF-Test(".to_string()),
        ("ZFrac1/2".to_string(), "ZFrac1⁄2".to_string()),
        ("ZFrac1/3".to_string(), "ZFrac1⁄3".to_string()),
        ("ZFrac1/4".to_string(), "ZFrac1⁄4".to_string()),
        ("ZFrac1/5".to_string(), "ZFrac1⁄5".to_string()),
        ("ZFrac1/8".to_string(), "ZFrac1⁄8".to_string()),
        ("ZFrac1/10".to_string(), "ZFrac1⁄10".to_string()),
        ("mathprintbox".to_string(), ".".to_string()),
        ("n/d".to_string(), "⁄".to_string()),
        ("Un/d".to_string(), "󸏵".to_string()),
        (">n/d<>Un/d".to_string(), "►n⁄d◄►Un⁄d".to_string()),
        (">F<>D".to_string(), "►F◄►D".to_string()),
        ("Sigma(".to_string(), "Σ(".to_string()),
        ("[MATHPRINT]".to_string(), "MATHPRINT".to_string()),
        ("[CLASSIC]".to_string(), "CLASSIC".to_string()),
        ("[n/d]".to_string(), "n⁄d".to_string()),
        ("[Un/d]".to_string(), "Un⁄d".to_string()),
        ("[AUTO]".to_string(), "AUTO".to_string()),
        ("[DEC]".to_string(), "DEC".to_string()),
        ("[FRAC-APPROX]".to_string(), "FRAC-APPROX".to_string()),
        ("[STATWIZARD ON]".to_string(), "STATWIZARD ON".to_string()),
        ("[STATWIZARD OFF]".to_string(), "STATWIZARD OFF".to_string()),
        ("plottinydot".to_string(), ".".to_string()),
        (
            "Quartiles Setting...".to_string(),
            "Quartiles Setting…".to_string(),
        ),
        ("u(n-2)".to_string(), "u(𝑛-2)".to_string()),
        ("v(n-2)".to_string(), "v(𝑛-2)".to_string()),
        ("w(n-2)".to_string(), "w(𝑛-2)".to_string()),
        ("u(n-1)".to_string(), "u(𝑛-1)".to_string()),
        ("v(n-1)".to_string(), "v(𝑛-1)".to_string()),
        ("w(n-1)".to_string(), "w(𝑛-1)".to_string()),
        ("u(n)".to_string(), "u(𝑛)".to_string()),
        ("v(n)".to_string(), "v(𝑛)".to_string()),
        ("w(n)".to_string(), "w(𝑛)".to_string()),
        ("u(n+1)".to_string(), "u(𝑛+1)".to_string()),
        ("v(n+1)".to_string(), "v(𝑛+1)".to_string()),
        ("w(n+1)".to_string(), "w(𝑛+1)".to_string()),
        ("SEQ(n)".to_string(), "SEQ(𝑛)".to_string()),
        ("SEQ(n+1)".to_string(), "SEQ(𝑛+1)".to_string()),
        ("SEQ(n+2)".to_string(), "SEQ(𝑛+2)".to_string()),
        (">DMS".to_string(), "►DMS".to_string()),
        (">Dec".to_string(), "►Dec".to_string()),
        (">Frac".to_string(), "►Frac".to_string()),
        ("->".to_string(), "→".to_string()),
        ("^^r".to_string(), "ʳ".to_string()),
        ("^^o".to_string(), "°".to_string()),
        ("^^-1".to_string(), "⁻¹".to_string()),
        ("^^2".to_string(), "²".to_string()),
        ("^^T".to_string(), "ᵀ".to_string()),
        ("^^3".to_string(), "³".to_string()),
        ("R>Pr(".to_string(), "R►Pr(".to_string()),
        ("R>Ptheta(".to_string(), "R►Pθ(".to_string()),
        ("P>Rx(".to_string(), "P►Rx(".to_string()),
        ("P>Ry(".to_string(), "P►Ry(".to_string()),
        ("[i]".to_string(), "𝑖".to_string()),
        ("|E".to_string(), "ᴇ".to_string()),
        ("theta".to_string(), "θ".to_string()),
        ("<=".to_string(), "≤".to_string()),
        (">=".to_string(), "≥".to_string()),
        ("!=".to_string(), "≠".to_string()),
        ("squareplot".to_string(), "□".to_string()),
        ("crossplot".to_string(), "﹢".to_string()),
        ("dotplot".to_string(), "·".to_string()),
        ("pi".to_string(), "π".to_string()),
        ("'".to_string(), "\'".to_string()),
        ("~".to_string(), "⁻".to_string()),
        ("sqrt(".to_string(), "√(".to_string()),
        ("cuberoot(".to_string(), "³√(".to_string()),
        ("e^^(".to_string(), "𝑒^(".to_string()),
        ("10^^(".to_string(), "₁₀^(".to_string()),
        ("sin^-1(".to_string(), "sin⁻¹(".to_string()),
        ("cos^-1(".to_string(), "cos⁻¹(".to_string()),
        ("tan^-1(".to_string(), "tan⁻¹(".to_string()),
        ("sinh^-1(".to_string(), "sinh⁻¹(".to_string()),
        ("cosh^-1(".to_string(), "cosh⁻¹(".to_string()),
        ("tanh^-1(".to_string(), "tanh⁻¹(".to_string()),
        ("smallL".to_string(), "ʟ".to_string()),
        ("xroot".to_string(), "ˣ√".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    conversions
}
