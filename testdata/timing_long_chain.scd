<?xml version="1.0" encoding="UTF-8"?>
<SCL version="2007" revision="B" xmlns="http://www.iec.ch/61850/2003/SCL">
  <Header id="timing-test-station" version="1.0" revision="A"/>
  <Communication>
    <SubNetwork name="GOOSE-NET" type="8-MMS">
      <ConnectedAP iedName="PROT-LINE1" apName="G1"/>
      <ConnectedAP iedName="PROT-LINE2" apName="G1"/>
      <ConnectedAP iedName="PROT-BUS1" apName="G1"/>
      <ConnectedAP iedName="PROT-TRANS1" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE1" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE2" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE3" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE4" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE5" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE6" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE7" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE8" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE9" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE10" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE11" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE12" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE13" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE14" apName="G1"/>
      <ConnectedAP iedName="LOGIC-GATE15" apName="G1"/>
      <ConnectedAP iedName="BCU-BAY1" apName="G1"/>
      <ConnectedAP iedName="CB-BREAKER1" apName="G1"/>
      <ConnectedAP iedName="CB-BREAKER2" apName="G1"/>
    </SubNetwork>
  </Communication>

  <IED name="PROT-LINE1" type="LineProtection">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <GSEControl name="GOOSE_TRIP_L1" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-01</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="PROT-LINE2" type="LineProtection">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <GSEControl name="GOOSE_TRIP_L2" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-02</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="PROT-BUS1" type="BusProtection">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="PROT-LINE1" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_TRIP_L1"/>
          </Inputs>
          <GSEControl name="GOOSE_TRIP_B1" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-03</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="PROT-TRANS1" type="TransformerProtection">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="PROT-BUS1" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_TRIP_B1"/>
          </Inputs>
          <GSEControl name="GOOSE_TRIP_T1" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-04</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE1" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="PROT-TRANS1" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_TRIP_T1"/>
          </Inputs>
          <GSEControl name="GOOSE_G1" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-11</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE2" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE1" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G1"/>
          </Inputs>
          <GSEControl name="GOOSE_G2" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-12</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE3" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE2" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G2"/>
          </Inputs>
          <GSEControl name="GOOSE_G3" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-13</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE4" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE3" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G3"/>
          </Inputs>
          <GSEControl name="GOOSE_G4" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-14</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE5" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE4" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G4"/>
          </Inputs>
          <GSEControl name="GOOSE_G5" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-15</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE6" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE5" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G5"/>
          </Inputs>
          <GSEControl name="GOOSE_G6" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-16</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE7" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE6" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G6"/>
          </Inputs>
          <GSEControl name="GOOSE_G7" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-17</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE8" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE7" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G7"/>
          </Inputs>
          <GSEControl name="GOOSE_G8" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-18</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE9" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE8" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G8"/>
          </Inputs>
          <GSEControl name="GOOSE_G9" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-19</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE10" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE9" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G9"/>
          </Inputs>
          <GSEControl name="GOOSE_G10" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-20</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE11" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE10" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G10"/>
          </Inputs>
          <GSEControl name="GOOSE_G11" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-21</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE12" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE11" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G11"/>
          </Inputs>
          <GSEControl name="GOOSE_G12" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-22</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE13" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE12" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G12"/>
          </Inputs>
          <GSEControl name="GOOSE_G13" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-23</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE14" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE13" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G13"/>
          </Inputs>
          <GSEControl name="GOOSE_G14" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-24</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="LOGIC-GATE15" type="LogicGate">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE14" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G14"/>
          </Inputs>
          <GSEControl name="GOOSE_G15" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-25</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="BCU-BAY1" type="BayControlUnit">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="LOGIC-GATE15" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_G15"/>
            <ExtRef iedName="PROT-LINE2" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_TRIP_L2"/>
          </Inputs>
          <GSEControl name="GOOSE_BCU_OUT" datSet="DS">
            <GSE><Address><P type="MAC-Address">01-0C-CD-01-00-30</P></Address></GSE>
          </GSEControl>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="CB-BREAKER1" type="CircuitBreaker">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="BCU-BAY1" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_BCU_OUT"/>
          </Inputs>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

  <IED name="CB-BREAKER2" type="CircuitBreaker">
    <AccessPoint name="G1">
      <Server><LDevice inst="LD0">
        <LN0 lnClass="LLN0" inst="">
          <Inputs>
            <ExtRef iedName="PROT-LINE2" apRef="G1" ldInst="LD0" lnClass="LLN0" doName="Trip" serviceType="GOOSE" cbName="GOOSE_TRIP_L2"/>
          </Inputs>
        </LN0>
      </LDevice></Server>
    </AccessPoint>
  </IED>

</SCL>
