use crate::math::field;
use crate::utils::{ as_bytes };
use sha3::Digest;

// CONSTANTS
// ================================================================================================

// Exponents for S-BOX (Poseidon, Rescue, GMiMC) and inverse S-BOX (rescue)
const ALPHA: u128 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;

// MDS matrix used by Poseidon and Rescue
const MDS: [u128; 36] = [
     34702391375697798808541201166389247321, 292720401120629668097050277338444166479, 252221686506898646925660607780980529565,   1545301432720594930091500405440765270, 249229091188143033873076468277345141138, 220001593723324427188563221285612032538,
    223274184432289781839114239013770504955, 330042960751289206923775620692185805456,  68147806084648525660922442535124284349, 170632587193822854126540173326689266153, 250033902372207462477717017592730125263, 241770281110130121239200125437407593586,
     59697037488579951129595490016876870776, 173037025415440639734730939871096987969, 244520331803890388707106378055030145592,  34432552219210978837375640622811234255, 224883744083395074894597669527169800639, 118987613174044827738657284387435362389,
    178405816334148045547444196947551204988, 329492269239016599078865693624718656026,   5932101030068686798137276449370802175,  82061764821777249869371835777715849101, 306668779179264388848571277248016578389, 111826390250505084847238806440816480816,
    311483585658925440282415980040674425324, 145759142525852648614462741190071048842, 298009719049064449565063658657087558281,  20897600766797241015108657845791963643,  10708575082009935910638872278310543860,   1925891850054765549789187338759995098,
    215280917571128372543200644166029121446,  95409967251054914374823434415713274752,  47264968375673684314231886553980659586, 324414896710549426218067045352609729751, 134533192639212680415562336758249126966, 113819576569856286031671903516923963713,
];

// Round constants for all hash functions; Poseidon uses all constants, Rescue the first 120, GMiMC uses the first 166
const ARK: [u128; 546] = [
     73742662193393629993182617210984534396, 158538539401072639862099558319550076686,  82429262549299942290847183493004485261, 310538827479436149892724250590698914519, 190338348930091047298074165559397264378, 289278996656706117461857789813498821934,
     53265540956785335308970867946461681393, 221096166077280180974764042888991644280, 135862987622353414661673448620033990934, 158907965876520949616863328303176330572, 274633988293091071340356635555807179190, 274604860873273636237081114376077113475,
     14395595548581550072136442264588359269,  58496669788416466040038464653643977917, 189653807408664613044858917026657980625, 230609671293877243511889006223284127479, 178527953570703982986577498890483203023, 126000924558481152083098962591383883438,
    122001776241989922016768881111033630021,  59235259390239124162891762278360245334,  89333775516890774827962437297764936547,  32424193637360906576442956294452323288,  62033181748425106711292370817969454146, 129877116445533126989528570413807277693,
     60517382118002481956993039132628798754, 337725857612570850944445340416668827103, 151495710594170316099539790651453416361, 250107916917535224528378129994943394294, 300207915911051460908298688414919921093, 172066229584406173063202914726937339958,
    242872884766759335785324964049644229294, 232074846252364869196809445831737773796, 287288998844960276649854461883880913666, 138628264101912804813977210615833233437, 279904213525927510712810228623902377237, 298530663250990395227144225232608384365,
      4363347423120340347647334422662129280,  50018412546799023168899671792323407156,  78065099645540746831460653583134588104, 265168486075436613449458788630803272512, 169061616865327291005064664270275836534,  16989575615175240495557720305287640349,
     36224510031696203479366212612960872957, 166545598284411242433605578379265360252,  55063854082489962294956447144901184837,  69216162599706897556278776240900218374, 109862755442048596627938134642975399668, 102835474498154050313290986853294842906,
     48405253030503584410290697712994785780,  41491163124497803255407972080635378902, 331958862978706756999748973740992156929, 189445283838085809052254029811407633258,  26236509279945457822369793146288866403, 297928660776980173370496618733852490961,
     81691558114273932307586556761543100315, 302719082300742526890675313445319567341,   8168599451814692118441936734435571667, 233141108584353453034002234415979233911,  12234569840122312404615178877814773825,  96037481352786813748421760769380383926,
    315851285839738308287329276161693313425, 193135973972933518870828237886863798021, 166002344081927954304873771936867289851, 214406010671246827947835794343033790693, 170640173476284978302806154399958141555,   2818165229115014774032882127170013258,
    326468515245013538774703881972225680443, 230635877078223923040415038811686445073, 225556280578098393163620719229418290860,  11153792801390339798262783617007369172, 209040028248304238735923683513240525194, 293027053537479076557105009345927645442,
     43697512293048123577843997788308773455, 138405289600908304802269329797084135857, 234470815157983004947611441850027217492, 118114082982223329826045602989947510129, 334227022756371766478760448625337379424, 249369722351358137898587699909312963803,
    311182552853825261047305944842224924215, 185089342855265166563915858025522983409, 188323096432976273265052369652285099186, 263157893448998999306850171729945394432,  25509259982013669682461356932775370545, 300544292992993952360719000252205715076,
     23833044413239455428827669432473543240,  43421407022492486112194101527865465264,  77086595049850596660690999278719011720, 284751400376525550861233017183497639371, 258001239441974384951891541079242930440, 323117003802246814764810890058143344905,
      7640791703119561504971867271087353186,  62365388436267647533064120634464266870, 219177966622498447376602481443936826442, 267697496976874759865163284761384997437, 170024582242541755392979256646565617273, 243579355010018669877160932197352017974,
    294241649061853322876594266104693176711, 116358578177298194933445426886059838431, 153964862116746563988492365899737226989,  93028746118909893246237533845189074002, 179443458614881887600494128053111694648, 339223760157195739332845857285008200423,
     37163237225742447359704121711857363416, 161426242690584918941198733450953748769, 171502007855719014010389694111716628578, 185875552438512768131393810027777987752, 108165142884901978856319583750672324489, 208632865147351209340449219082125897333,
    122453723578185362799857252115182955415, 228752352631998151610775212885524543283,  69476260396969790693146402021744933499, 185941035889835671403097747661105595079,  97063282200318501142854934314343169049,  96675618862527967726114378655626650641,
     45955200056324872841369110391855073949, 182846621472767704751329603405195985261, 154737674033120948700227987365296907637, 253746202714926890236889780444552427226, 261286087759526359216271155361018330507, 162892536327655189685235410342890574896,
    118224404177203231307646344308524770691,  61911644581679112386499312030413349074, 321756280164272289841871040803703440350, 101396399019525872501663112616210307683,  67833038363599207475373040930824843019, 196910153233132861881308509456401645140,
    334905318181122708043147970432770442813, 191090374127295994022314014407997806335, 131528659800906379821588177210148124019, 215901816653704881294214215068798346060,  56878992720628535103195481580617360771, 281841826874183647567546019531929972702,
    151456178618798089303785904835852898400,  59079983632109588980783021622461415033,  80761083418432627134481526210881542477, 201416315867789883891889554246377963162, 198852036109370286966576164360266278255, 155276073049009029667373106803046514344,
    158324780313294970656577210958221752332, 193859304217638223479173371326185841274, 250524460337537482950569449224813700391, 251801358233276697762579413801911171612, 174521831193496100673067735908873646985, 152642017050116048509158960350000858013,
     94987431711345870355583825474329298047, 280938106646730498467301259432184740730, 230491494516843960542668710050516211970, 192826288785653777157020265215517987662, 251654188127562510403516067236333482372, 286456894851095755022390967246767421000,
    314293870425266938862923101612602484635,    679464766810703810097965767355062873,  45314269339319734203127091458645722622,  15885268928012076989988458786521561031,  48056343894932757577046683797067209079, 215531716255970146473658338852472046173,
    153975056764703018977481562856167540343, 150345637803188209699415557320545415720,   2780762044206215421580528389917005833,   3463161311202689884181131747640413605, 306942787541210815164178987028698818659, 324452408864695917006896030536225525119,
    321383880935903155966493388921501530915, 139823638104054506965247243102295231737, 165769058045453677221497711462583950139,  79003969367131068546865741459306118251, 156642260202818413362503062578539720517, 314094406162389098987684450322979120529,
     50057060110310193394516504439805601601,  53655583013674525883209345165753178194, 259395388889719671782653113655841647262,  69521903572951337445452502748565566496, 251616653853928459967283575542057535293, 114910730596486251472791631840513265074,
    101740347373933108709122003348416870840, 126806292806004264446745284405742612689, 219135320838134930923443959673366229256, 301962999029915705994021697766389081208, 188741644029927191719040650968720800409,  81795345404219176616297063519210464031,
     80845608757236703492016225128275757615,   9602891270757320013616862490986026227, 286172465494565666647121151879345971089,   9094956230559373758985913855137693312, 281428110117091114144446350524650424481,  22603524397731600512825466576357638930,
    209519938465996994070512842713405349097, 160806286415414414379046661006476545066, 147904845125332345734618546117273133070, 144516981820451119929097195276243798745,  64627937813848943279040280988334503406,  63900149356112496372337283043133097338,
    106431522239104133028209232303575045999,  18613953306501380292821686668070800928,  89671526728869908827203081623962249311, 107298672259979748821487552283470868404, 325995509498559936941639775857623491149, 296603240634431343398772857230532468105,
    335773254333882338763168611010748689291, 111256818554966355701207525051891006240,  32702862174087318754748457941525987872, 110565758013003762337520720256392465392,   9721331932840608097576034919320981735, 315354270802025154630993621253906741728,
     71699845531121675857954598186749662284, 195679664846665933651707201114088711381,  66708296842161636650978457391386571852, 150498377771687844793683648285177451802, 312357873092635365762349741128765299483, 123012818925347408581142191691408689823,
    244302174841298602345381639425177304690,  59108717575800760137095729886266063132,  86499217558887308982356554421154086418, 175199507444641968880385252963717485585, 258606324890711530268005798565861051635, 163039673396503991004505529151087013530,
    205259811453788512829856730355148044152,    940137264911903668894058445547083903,  98552119713558805377144327892189294864, 175181221027810746545027029244857962693, 224460018186632104852175202637252669484, 301313391419233287870843537857121704726,
     85546281816802285708851243544687494607, 126821286381895205559298478188749073506, 202196034787035338340285083273371951808,  80945288111596572026556639362541218751, 228205360870734533157252128330721170842, 226702480604300898773897688441681490946,
    279976288469848092836635937130584488114,  62692921885099987314245775046521135879, 244057116335001059191351694477099237233, 163685250545158312857507270424156986799, 291070448792013322374900311397232779119, 107293897301257482237903809190949679199,
    119620041655308890504806386952183174823,   2247532578875052765096486699976115620, 105379696137203053082432023865153571937, 324728240036842947889881077887444768135, 276978732556895888452684475234835494718, 301960936895149185849956478974345721321,
    311133079471813191460720785542148473136, 297437347003243432950563061985581138691, 111262063949258149672778785425054700286, 303466690424641670273869498914969399126, 247465621844917321033338740570614577825,   1374733844625689353717215212435018104,
    287640517402395554005351257579796043452, 228318216733403765889973300042783374106,  70456617577364025689249149449176507511, 307090744331035785628907898926970490358,   5530667761046984427877799832856199991,  10724875773477085713462836852369053471,
    176302823634403985888488898352905346501, 317494385291032789149148931168020364876, 228066653254555337126585656299527917150, 114830202638019614604402133991667569495, 251959833874619709417795999287362902241, 217574360855444413442626262686717254920,
    103153761598852200166498082364880371919, 206296310276497992533071132508328325791,  26116253319347397373766203089063795739,  53780491522532720338269555748204746051, 136961045709807633025878790807556086436,  75173470908546803503744792000944490637,
     37778291530386517249889893643084137980, 169650781125828167991126280009081408652, 210905712276460680045153301307122633497, 285098910112192674011234690198795125806, 183229478952162608454215404367033299744, 277279550209221864218089088043739239284,
    263462769752377698391281531765633526655, 263404873985070527351017576735139793112, 206250040440536706649144436951899925845, 110502129694052414993211410654596214536, 125764326822280689715888535005920487343, 127131966210009417920124380892125016044,
    299335201413142816275679686070565419423, 193462370101944161229708956687775582276, 339712081626991640189890263671754636785,  80935912067176468011267034970122382410, 148651654008632859608217659776851450815, 121552742453381425065143225714876875320,
    335080263538966226432363562764273876868, 228966754899363243140946822166584090513, 326053791724373498851900767391498727646, 233925655915766936988011541306352999728,  43678072028144116086969416316467722814, 135904063707066228297060847873007407363,
     34239160820156082033345865764729053215, 161009627837431049306010856832995244421, 237949488275099172550759048960495832323, 146751663365123271283839229992597852324, 180513984706521440378726894237980229164,  96271186465923623435751072767559836779,
    277951578890827342549203338995849052720, 324371553867454630129994975725453039602, 321861938918506427282848103901728653196,  14524480921222562574232083474438523273, 252195141029120080807540466495213269030, 117984746593979114603377131809985488042,
    304210455843703776864430370154744246929, 239841673424067115896811717873692351024, 110099907542944359613842158648834628683, 146050557031273982798598436135706239456, 327737075204378381917933835057043376766, 179045230306204925470788469055931944493,
     54301906275050665323788919128955939892,  40527201010851201196502280992391562055,  83492597088486156416619383709849616961, 182491474014405414819021456790193533016, 188028798865797172830548020924529878939, 117428702618273586737145560834940585023,
    217495917467932704796601363421290653539, 317390121655364616950607644288947849548,  45488171013895871645086548693558641780, 147060038311178460034101203742764740514, 152669194770948361885110296563708589660, 211671885115459125313490226604694265585,
     55005805415527437494232043262441786103,  42833546678096542131959813645866639891,  25055583738648539912651453455808049593, 290034874305086232344521951907523582946,  45887585381077422685971845069490380706, 195936630118470129696376754971190971674,
    286622166073614683380051204489950971432, 337682822486819352723331194637539882419, 202340415879953952508595626231700088322,  49885637902973857688356684687002818169,  65686521881581877010889709104999812880, 223573080118566074706325252736994835261,
    130503829121681867733555657295425478630, 128965743326483613268554364173687634939, 184045330236562026990115078673252453732,   6883151615947021135690150480138611759,  63984568530779210847697663800972184670, 118182159363671899850376794173625379549,
    210811624119018247329924906557627214750, 267331837928693215458178147572485691685, 263641541351342801173666018950374731093, 206763234226070622384307818869452987351,  68163832080088800524442156589605366142, 315202097235742938698865163927033676964,
    241076627417316318657648239838578198924, 257169265034746529768556525401893992305, 316805344666553230539045458349089930630,  70555767079188217319011780321992172556,  22154958064252430080246887473650149079, 334475305353973049011206849400971303128,
     17794023428273411626675250981282256961, 338573472003585450680269841729113359058, 105576764334125716454523429552809730050,  22986585681141743166208290033349726582, 238329839498982764463253385303742697402, 165724845017477397924657903357266325117,
    260320390405884689419817099271942871640, 211392115666800584965626882987757747377, 216821467867088606674954726717599860933,  73340150412887029223743023571613403405,   9358737102138509230110387006074816934, 182902072041662112618892041674535009314,
    123257725746114211602862787729869402971, 176167395494299753030163959654923110840, 165832484276968874206326273962354469568, 160791980177892456748538954784038583895, 179605862364020419638477202924470422213, 273164167138629228607458244599817947373,
     62220443656210044015753320681968818344, 105567816043010151482100586774982212702, 239189297520521225311545689807638971547, 209958267959236160146364000425083656942, 311934939892487933223008180468154430869, 302135822688262131102756383936000484299,
    328648222412605551116426768977111517882, 176995376966999423966691088355581116693, 284938810260567776086300305667938659580, 219705729239819588852659093074897696035, 232471304599992224598521057628192726483, 227108371256629370667865485639030983654,
    312887158225025475375528472817828549875,  61910988495463748105983786691401080077,  73009294410858875281436498499831340417, 272774120947363811195528813885457626246, 235210736632368495546976222469276777221,   2401979307381420410010413351033437540,
    215506975211752641229724295212587067945,  82548309266515774913171132417732204255, 246666910673797201670146830942877079943,  96152238995201732273428024483710682032,  74372037516521130504996682378851473951, 123545031554761653580808608276017133148,
    273310685081032943244306142452922509359, 108412943540610636599422106647386972478, 307424926349213952068541679277713654334,  15284196567508759396459300476198117443, 285117880002813460592404101243527817233, 269355483573778412819110095696171261784,
    143801553346262339655414641085839374921, 195161331664270076858268337818421867699, 188454493685247757769950688001954570222,  74487389285628440110948714364114439964, 113511737332142390171568183289687426702, 135400104132532503238745075054145442778,
    302834257254272780859236393724387622308, 236575715845323377563352316082536718395,  29353015611619933087205366954618863770,  38331809676466082292168595028997628084,  35907080069482466201191610457248445073, 190509880765785040087704445538370918814,
     31101218862327739622459961560844817192, 229544908045726883197266170583756750927,  98569259564117983374188250873343683122,  24605082859506616788602507283448067176, 110941689818073301623398693297964529658, 222669826886726755394856014991811136408,
    290802854577534731604380315779695506435, 114573200723897487731664406668147024884, 308791282265633200464847985629599353829,  18107126277889198482164647813020319128, 102571075486197518864912871663095788974,  57417927276153434488925361498123836398,
    245326353537118290058814317615491764292, 116400766382144680481009983964929813515, 183681095705113406461054756793564692986, 247329694776621706020927219393650069400, 263688895342890989028076326642790593393,  70278982303178609569330776226815738412,
     97685778720714285235572214011724858927, 300021815700282393708667172371367495106, 265558123755588632448501911255129527135,  78359592926889865287558330417498309169, 171034785651384700790731255569943130567, 139759466635658285631009883653934057017,
    236145503252787556253387419145986201156, 213594960361236746901001334823545872650, 322238219636156644726716766642638091380, 151277159690531100514318314659481490879, 184879794769790841475773538950259191435, 244336344186062431970803456655058917876,
    139832620409466154282819796195918383416, 135160133646554875200519455616904492695, 258529116902563932646652085100826912069, 265322218202912614702868011342195476375,  39965371661683042252844598281945186988, 331438576076703351037992000550041149470,
    156355538718444986559558402064857172565, 165047391498981587341575215600404456178,  66904188157758101999288257991820223574, 226081959544077704758412894459493575594, 160166462388273925999180814214089516948,  99170464289488265625452394802798309788,
    295842130581607230528368723530787072385, 141827583334048437871853766280622502313, 295642148914908568580514812563022820900, 227889510170594860450976791377979007194,  33135908427465012647901818755217265183, 147613642541618622439708039978164466061,
     82106392893059679914703393139586191415,  44818109097664825546792183505550262380, 247697001018151660543208829598508116358, 246979052117841512114760280292975520298, 326830000802068150748592893524102464750, 175231986331864088111786314037499741670,
     60297779531378695135373224863136171940, 106059462955266823324801733430448941054,  74223258840977696662245827329777045621, 132935739869421167390794370566676155228, 104814145035375707654234601437450602213, 112584029435012872330783102516413502835,
    314583297983585083955724995456263708474, 216372387025797678517644831164184942170,  31657845941424345924069684422666464293, 146009154431781185618193961004753465884,  61586267315991147424740012454009471471, 119544578111151718905950991212589071388,
    166762277097015590138038345499327042589, 264058601838195424043179407868859932393, 254207058887987123284793311724299350939, 162268154827601362274179486451565868008,  67194299462791376670391676843849320501,  46719822879574814361083170527780394518,
    203897786115203446815354249067374736411,  79107619414887597284515088993722422777, 252239895378385090359984242337276862568,  82687556184912016560867940903470841506, 149027331216835058049504154297196492186, 278743798672672026953537260756416293111,
     60717870482613277319839291959228618779,  23534969026060982707070884120357068566, 273064765018585860048928078946408407125, 114911047278322484566552206257344434823, 211002437221198815521099473935393417912,  69183998213453996169922366242493114178,
    264474942684940483377374652329000223495,  40443565488589778697153140829803233362, 318236855001371491410061863981186622893,  95137115567713482334442933263753699151,  94010814144364083412660467733267504803,  57832467619251300825118569407390307616,
    241646763657955656769668332970603164850, 328655501188286296926130680363931665471, 292331577442880542099569289553512900473, 210347505804616330182307728243620642940, 264148679859805516629557364715507257392,  86734318673331003889354566816231041663,
    121096727880836913794832542529061580941, 306658769045859709372390017627291895631, 298214129710003851648333027731376339958,  36766969337462686755233408536107486598, 222680061782005334265047766565321604289, 245433354868861787897264737776353949683,
     92910101379608621912766210543385190585, 154818446593425285823280086870305670006,  46851151721525371384218733580259577305, 299122313161604326021516682466256829643, 256349382350306902334274088930618519275,  37405110781854006830278667314186353355,
    331150331241631368269738414686017964126,  32416560807569586010577128444412204086, 156479903527413035151695552698190648278,  34007842601924530794411311500972328065, 228261733027121859429901027919740446798, 225302516595055234760535576897586315501,
    173210506970404923632084506466389983015,  87797296672990884717022893688362999741, 113391612689561810119594900137743839681,    990798401336824450528687493413224864, 222785463985560370324371058457334918995, 174748209710356985086197364309198225955,
     15874842091358438228362466024843871377,  68173606781408595809053358079143778257,   4459673975578626169929706739533339573, 230198625303475744316545739743244619342, 296919342681064113744581717848983415815, 201876812203759264411864483577380626017,
     21406475271329194697594404366944715754,  47497691124224347780157019996806776539, 121640897701002077408881344747127276038, 211481365482692155719117019780628509988, 102481962997731197177309997815733138785, 191745802290321837559897992968281267420,
    309924869726711895128876537262983061064, 112525655489195767040829402475371392715, 170125078093491169855652980913547448221, 294706318451527567281025752400950837744, 206842437959190858648222146486983588628, 285833121646488198141311208833491008916,
];

// ------------------------------------------------------------------------------------------------
/// Poseidon hash function
pub fn poseidon(values: &[u8], result: &mut [u8]) {
    debug_assert!(values.len() <= 64, "expected 64 or fewer input bytes but received {}", values.len());
    debug_assert!(result.len() == 32, "expected result to be exactly 32 bytes but received {}", result.len());

    // copy values into state and set the remaining state elements to 0
    let mut state = [0u128; 6];
    let state_bytes: &mut [u8; 64] = unsafe { &mut *(&state as *const _ as *mut [u8; 64]) };
    state_bytes[..values.len()].copy_from_slice(values);

    // execute round function 48 times
    for i in 0..91 {

        add_constants(&mut state, i * 6);

        if i < 4 || i >= 87 {
            // full round
            apply_sbox(&mut state);
        }
        else {
            // partial round
            state[5] = field::exp(state[5], ALPHA);
        }

        apply_mds(&mut state);
    }

    // return the result
    result.copy_from_slice(as_bytes(&state[..2]));
}

// ------------------------------------------------------------------------------------------------
/// Rescue hash function
pub fn rescue(values: &[u8], result: &mut [u8]) {
    debug_assert!(values.len() <= 64, "expected 64 or fewer input bytes but received {}", values.len());
    debug_assert!(result.len() == 32, "expected result to be exactly 32 bytes but received {}", result.len());

    // copy values into state and set the remaining state elements to 0
    let mut state = [0u128; 6];
    let state_bytes: &mut [u8; 64] = unsafe { &mut *(&state as *const _ as *mut [u8; 64]) };
    state_bytes[..values.len()].copy_from_slice(values);

    // apply round function 10 times
    add_constants(&mut state, 0);
    for i in 0..10 {

        // step 1
        apply_inv_sbox(&mut state);
        apply_mds(&mut state);
        add_constants(&mut state, (i * 2 + 1) * 6);

        // step 2
        apply_sbox(&mut state);
        apply_mds(&mut state);
        add_constants(&mut state, (i * 2 + 2) * 6);
    }

    // return the result
    result.copy_from_slice(as_bytes(&state[..2]));
}

// ------------------------------------------------------------------------------------------------
/// GMiMC_erf hash function
pub fn gmimc(values: &[u8], result: &mut [u8]) {
    debug_assert!(values.len() <= 64, "expected 64 or fewer input bytes but received {}", values.len());
    debug_assert!(result.len() == 32, "expected result to be exactly 32 bytes but received {}", result.len());

    // copy values into state and set the remaining state elements to 0
    let mut state = [0u128; 6];
    let state_bytes: &mut [u8; 64] = unsafe { &mut *(&state as *const _ as *mut [u8; 64]) };
    state_bytes[..values.len()].copy_from_slice(values);

    for i in 0..166 {
        let s0 = state[0];
        let mask = field::exp(field::add(s0, ARK[i]), ALPHA);
        for j in 1..6 {
            state[j - 1] = field::add(mask, state[j]);
        }
        state[5] = s0;
    }

    // return the result
    result.copy_from_slice(as_bytes(&state[..2]));
}

// ------------------------------------------------------------------------------------------------
/// Wrapper around blake3 hash function
pub fn blake3(values: &[u8], result: &mut [u8]) {
    debug_assert!(result.len() == 32, "expected result to be exactly 32 bytes but received {}", result.len());
    let hash = blake3::hash(&values);
    result.copy_from_slice(hash.as_bytes());
}

/// Wrapper around sha3 hash function
pub fn sha3(values: &[u8], result: &mut [u8]) {
    debug_assert!(result.len() == 32, "expected result to be exactly 32 bytes but received {}", result.len());
    let mut sha256 = sha3::Sha3_256::new();
    sha256.input(&values);
    let hash = sha256.result();
    result.copy_from_slice(hash.as_ref());
}

// HELPER FUNCTIONS
// ================================================================================================
fn add_constants(state: &mut[u128; 6], offset: usize) {
    for i in 0..6 {
        state[i] = field::add(state[i], ARK[offset + i]);
    }
}

fn apply_sbox(state: &mut[u128; 6]) {
    for i in 0..6 {
        state[i] = field::exp(state[i], ALPHA);
    }
}

fn apply_inv_sbox(state: &mut[u128; 6]) {
    // TODO: optimize
    for i in 0..6 {
        state[i] = field::exp(state[i], INV_ALPHA);
    }
}

fn apply_mds(state: &mut[u128; 6]) {
    let mut result = [0u128; 6];
    let mut temp = [0u128; 6];
    for i in 0..6 {
        for j in 0..6 {
            temp[j] = field::mul(MDS[i * 6 + j], state[j]);
        }

        for j in 0..6 {
            result[i] = field::add(result[i], temp[j]);
        }
    }
    state.copy_from_slice(&result);
}

// TESTS
// ================================================================================================
#[cfg(test)]
mod tests {

    use crate::utils::{ as_bytes };

    #[test]
    fn poseidon() {
        let value = [1u128, 2, 3, 4];
        let mut result = [0; 32];
        super::poseidon(as_bytes(&value), &mut result);

        assert_eq!([
            224,  9,  85,  92, 75, 117, 136,  23, 142,  67, 249, 199, 39, 177,  97, 129,
            93, 192, 153, 131, 76, 160,  94, 162, 200, 192, 187,   5, 159, 69,  48, 165], 
            result);
    }

    #[test]
    fn rescue() {
        let value = [1u128, 2, 3, 4];
        let mut result = [0; 32];
        super::rescue(as_bytes(&value), &mut result);

        assert_eq!([
            148, 191,  96, 185, 107, 196, 170,  28, 161, 214, 196, 211, 158, 111, 135, 32, 
            122, 173, 195,  37, 123,  60, 246, 104, 176,  53, 127,  67,  38, 208,  69, 54],
            result);
    }

    #[test]
    fn gmimc() {
        let value = [1u128, 2, 3, 4];
        let mut result = [0; 32];
        super::gmimc(as_bytes(&value), &mut result);

        assert_eq!([
            115, 208,  64, 41, 162,  43, 134, 243, 236,  80, 161, 106, 195, 234, 30, 26,
             71,  74, 255, 77,  41, 125,  25, 152, 162, 106,  65, 108,  84, 216, 37, 37],
            result);
    }
}
