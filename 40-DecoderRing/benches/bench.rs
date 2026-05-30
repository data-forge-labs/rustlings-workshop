use criterion::{criterion_group, criterion_main, Criterion};
use decoder_ring::{guess_shift, guess_shift_parallel};

static ENCRIPTED_TEXT: &str = "Ybza pz h wyvnyhttpun shunbhnl aoha pz nyvdpun pu wvwbshypaf. Dopsl paz bzly ihzl ylthpuz zthss, pa pz dpklsf ylnhyklk hz h jvvs shunbhnl. Hjjvykpun av aol Zahjr Vclymsvd Klclsvwly Zbyclf 2022, Ybza ohz illu aol tvza-svclk shunbhnl mvy zlclu zayhpnoa flhyz. Ybza ivhzaz h bupxbl zljbypaf tvkls, dopjo wyvtpzlz tltvyf zhmlaf huk jvujbyylujf zhmlaf, dopsl wyvcpkpun aol wlymvythujl vm J/J++. Ilpun h fvbun shunbhnl, pa ohz uva illu zbiqljalk av aol dpklzwylhk zjybapuf hmmvyklk av vskly shunbhnlz, zbjo hz Qhch. Jvuzlxbluasf, pu aopz isvn wvza, dl dvbsk sprl av hzzlzz Ybza’z zljbypaf wyvtpzlz.

Lclyf shunbhnl wyvcpklz paz vdu zljbypaf tvkls, dopjo jhu il klmpulk hz aol zla vm zljbypaf huk zhmlaf nbhyhuallz aoha hyl wyvtvalk if lewlyaz pu aol shunbhnl. Mvy lehtwsl, J ohz h clyf ybkptluahyf zljbypaf tvkls iljhbzl aol shunbhnl mhcvyz wlymvythujl vcly zljbypaf. Aolyl ohcl illu zlclyhs haaltwaz av ylpu pu J’z tltvyf zhmlaf pzzblz, myvt PZV J’z Huhsfghipspaf Huule av Joljrlk J, iba uvul ohcl hjoplclk dpklzwylhk wvwbshypaf fla.

Vm jvbyzl, huf shunbhnl thf mhps av spcl bw av paz zljbypaf tvkls kbl av ibnz pu paz ptwsltluahapvu, zbjo hz pu h jvtwpsly vy pualywylaly. H shunbhnl’z zljbypaf tvkls pz aobz ilza cpldlk hz doha paz jvtwpsly vy pualywylaly pz lewljalk av zbwwvya yhaoly aohu doha pa jbyyluasf zbwwvyaz. If klmpupapvu, ibnz aoha cpvshal h shunbhnl’z zljbypaf tvkls zovbsk il aylhalk clyf zlypvbzsf if aol shunbhnl’z klclsvwlyz, dov zovbsk zaypcl av xbpjrsf ylwhpy huf cpvshapvuz huk wylclua uld vulz.

Ybza'z zljbypaf tvkls pujsbklz paz jvujlwa vm vdulyzopw huk paz afwl zfzalt. H shynl whya vm Ybza’z zljbypaf tvkls pz lumvyjlk if paz ivyyvd joljrly, dopjo pz h jvyl jvtwvulua vm aol Ybza jvtwpsly (ybzaj). Aol ivyyvd joljrly pz ylzwvuzpisl mvy luzbypun aoha Ybza jvkl pz tltvyf-zhml huk ohz uv khah yhjlz. Qhch hszv lumvyjlz tltvyf zhmlaf iba kvlz zv if hkkpun ybuaptl nhyihnl jvssljapvu huk ybuaptl joljrz, dopjo ptwlkl wlymvythujl. Aol ivyyvd joljrly, pu aolvyf, nbhyhuallz aoha ha ybuaptl Ybza ptwvzlz hstvza uv wlymvythujl vclyolhk dpao tltvyf joljrz (lejsbkpun joljrz kvul lewspjpasf if aol zvbyjl jvkl). Hz h ylzbsa, aol wlymvythujl vm jvtwpslk Ybza jvkl hwwlhyz jvtwhyhisl av J huk J++ jvkl huk mhzaly aohu Qhch jvkl.

Klclsvwlyz hszv ohcl aolpy vdu tluahs zljbypaf tvklsz aoha ltivkf aol wvspjplz aolf lewlja vm aolpy jvkl. Mvy lehtwsl, aolzl wvspjplz afwpjhssf pujsbkl hzzbyhujlz aoha wyvnyhtz dpss uva jyhzo vy slhr zluzpapcl khah zbjo hz whzzdvykz. Ybza’z zljbypaf tvkls pz pualuklk av zhapzmf klclsvwlyz’ zljbypaf tvklsz dpao chyfpun klnyllz vm zbjjlzz.

Aopz isvn wvza pz aol mpyza vm adv ylshalk wvzaz. Pu aol mpyza wvza, dl lehtpul aol mlhabylz vm Ybza aoha thrl pa h zhmly shunbhnl aohu vskly zfzaltz wyvnyhttpun shunbhnlz sprl J. Dl aolu lehtpul sptpahapvuz av aol zljbypaf vm Ybza, zbjo hz doha zljbyl-jvkpun lyyvyz jhu vjjby pu Ybza jvkl. Pu h mbabyl wvza, dl dpss lehtpul Ybza zljbypaf myvt aol zahukwvpuaz vm bzlyz huk huhsfzaz vm Ybza-ihzlk zvmadhyl. Dl dpss hszv hkkylzz ovd Ybza zljbypaf zovbsk il ylnhyklk if uvu-klclsvwlyz, l.n., ovd thuf jvttvu cbsulyhipspaplz huk lewvzbylz (JCLz) wlyahpu av Ybza zvmadhyl. Pu hkkpapvu, aopz mbabyl wvza dpss mvjbz vu aol zahipspaf huk thabypaf vm Ybza pazlsm.";

fn guess_shift_single_thread(c: &mut Criterion) {
    c.bench_function("guess_shift", |b| b.iter(|| guess_shift(ENCRIPTED_TEXT, 26)));
}

fn guess_shift_multi_thread(c: &mut Criterion) {
    c.bench_function("guess_shift_parallel", |b| b.iter(|| guess_shift_parallel(ENCRIPTED_TEXT, 26)));
}

criterion_group!(
    benches,
    guess_shift_single_thread,
    guess_shift_multi_thread,
);

criterion_main!(benches);
