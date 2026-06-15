<?xml version="1.0" encoding="UTF-8"?>
<SCL version="2007" revision="B" xmlns="http://www.iec.ch/61850/2003/SCL">
  <Header id="test-station-001" version="1.0" revision="A">
    <History>
      <Hitem version="1.0" revision="A" who="SystemIntegrator" when="2024-01-15T10:30:00">
        <Text>Initial configuration for 500kV substation</Text>
      </Hitem>
    </History>
  </Header>

  <Communication>
    <SubNetwork name="GOOSE-NET-A" type="8-MMS">
      <ConnectedAP iedName="PROT-L1" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.101</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-L2" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.102</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-T1" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.103</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="MU-L1" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.104</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="MU-L2" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.105</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="BCU-1" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.106</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="MONITOR-1" apName="GOOSE-A">
        <Address>
          <P type="IP">192.168.1.107</P>
        </Address>
      </ConnectedAP>
    </SubNetwork>

    <SubNetwork name="SV-NET-A" type="8-MMS">
      <ConnectedAP iedName="MU-L1" apName="SV-A">
        <Address>
          <P type="IP">192.168.2.101</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="MU-L2" apName="SV-A">
        <Address>
          <P type="IP">192.168.2.102</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-L1" apName="SV-A">
        <Address>
          <P type="IP">192.168.2.103</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-L2" apName="SV-A">
        <Address>
          <P type="IP">192.168.2.104</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-T1" apName="SV-A">
        <Address>
          <P type="IP">192.168.2.105</P>
        </Address>
      </ConnectedAP>
    </SubNetwork>

    <SubNetwork name="MMS-NET" type="8-MMS">
      <ConnectedAP iedName="PROT-L1" apName="MMS">
        <Address>
          <P type="IP">192.168.3.101</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="PROT-L2" apName="MMS">
        <Address>
          <P type="IP">192.168.3.102</P>
        </Address>
      </ConnectedAP>
      <ConnectedAP iedName="MONITOR-1" apName="MMS">
        <Address>
          <P type="IP">192.168.3.103</P>
        </Address>
      </ConnectedAP>
    </SubNetwork>
  </Communication>

  <IED name="PROT-L1" type="LineProtection" manufacturer="ManufacturerA">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <GSEControl name="GOOSE_TRIP_L1" datSet="DS_TRIP" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-01-01</P>
                  <P type="APPID">0x0001</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">4</P>
                </Address>
                <MinTime unit="s">2</MinTime>
                <MaxTime unit="s">10</MaxTime>
              </GSE>
            </GSEControl>
            <GSEControl name="GOOSE_SIGNAL_L1" datSet="DS_SIGNAL" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-01-02</P>
                  <P type="APPID">0x0002</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">3</P>
                </Address>
              </GSE>
            </GSEControl>
          </LN0>
          <LN lnClass="PTOC" inst="1">
            <Inputs>
              <ExtRef iedName="MU-L1" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TCTR" lnInst="1" doName="Amp" daName="instMag.f" serviceType="SMV"/>
              <ExtRef iedName="MU-L2" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TVTR" lnInst="1" doName="Vol" daName="instMag.f" serviceType="SMV"/>
              <ExtRef iedName="PROT-T1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Block" daName="stVal" serviceType="GOOSE" cbName="GOOSE_BLOCK_T1"/>
            </Inputs>
          </LN>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="SV-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="MMS">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="PROT-L2" type="LineProtection" manufacturer="ManufacturerA">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <GSEControl name="GOOSE_TRIP_L2" datSet="DS_TRIP" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-02-01</P>
                  <P type="APPID">0x0003</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">4</P>
                </Address>
              </GSE>
            </GSEControl>
          </LN0>
          <LN lnClass="PTOC" inst="1">
            <Inputs>
              <ExtRef iedName="MU-L2" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TCTR" lnInst="1" doName="Amp" daName="instMag.f" serviceType="SMV"/>
              <ExtRef iedName="MU-L1" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TVTR" lnInst="1" doName="Vol" daName="instMag.f" serviceType="SMV"/>
              <ExtRef iedName="PROT-T1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Block" daName="stVal" serviceType="GOOSE" cbName="GOOSE_BLOCK_T1"/>
            </Inputs>
          </LN>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="SV-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="MMS">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="PROT-T1" type="TransformerProtection" manufacturer="ManufacturerB">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <GSEControl name="GOOSE_BLOCK_T1" datSet="DS_BLOCK" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-03-01</P>
                  <P type="APPID">0x0005</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">4</P>
                </Address>
              </GSE>
            </GSEControl>
            <GSEControl name="GOOSE_TRIP_T1" datSet="DS_TRIP_T1" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-03-02</P>
                  <P type="APPID">0x0006</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">4</P>
                </Address>
              </GSE>
            </GSEControl>
          </LN0>
          <LN lnClass="PDIF" inst="1">
            <Inputs>
              <ExtRef iedName="MU-L1" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TCTR" lnInst="1" doName="Amp" daName="instMag.f" serviceType="SMV"/>
              <ExtRef iedName="MU-L2" apRef="SV-A" ldInst="LD0" prefix="" lnClass="TCTR" lnInst="1" doName="Amp" daName="instMag.f" serviceType="SMV"/>
            </Inputs>
          </LN>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="SV-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="MU-L1" type="MergingUnit" manufacturer="ManufacturerC">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="SV-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <SampledValueControl name="SV_L1_CURR" smvID="MU_L1_CT" datSet="DS_CURR" confRev="1">
              <SMV>
                <Address>
                  <P type="MAC-Address">01-0C-CD-04-01-01</P>
                  <P type="APPID">0x4001</P>
                  <P type="VLAN-ID">200</P>
                  <P type="VLAN-PRIORITY">5</P>
                </Address>
              </SMV>
            </SampledValueControl>
            <SampledValueControl name="SV_L1_VOLT" smvID="MU_L1_VT" datSet="DS_VOLT" confRev="1">
              <SMV>
                <Address>
                  <P type="MAC-Address">01-0C-CD-04-01-02</P>
                  <P type="APPID">0x4002</P>
                  <P type="VLAN-ID">200</P>
                  <P type="VLAN-PRIORITY">5</P>
                </Address>
              </SMV>
            </SampledValueControl>
          </LN0>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="MU-L2" type="MergingUnit" manufacturer="ManufacturerC">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="SV-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <SampledValueControl name="SV_L2_CURR" smvID="MU_L2_CT" datSet="DS_CURR" confRev="1">
              <SMV>
                <Address>
                  <P type="MAC-Address">01-0C-CD-04-02-01</P>
                  <P type="APPID">0x4003</P>
                  <P type="VLAN-ID">200</P>
                  <P type="VLAN-PRIORITY">5</P>
                </Address>
              </SMV>
            </SampledValueControl>
            <SampledValueControl name="SV_L2_VOLT" smvID="MU_L2_VT" datSet="DS_VOLT" confRev="1">
              <SMV>
                <Address>
                  <P type="MAC-Address">01-0C-CD-04-02-02</P>
                  <P type="APPID">0x4004</P>
                  <P type="VLAN-ID">200</P>
                  <P type="VLAN-PRIORITY">5</P>
                </Address>
              </SMV>
            </SampledValueControl>
          </LN0>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="BCU-1" type="BayControlUnit" manufacturer="ManufacturerD">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst="">
            <GSEControl name="GOOSE_BCU_STATUS" datSet="DS_STATUS" confRev="1">
              <GSE>
                <Address>
                  <P type="MAC-Address">01-0C-CD-01-04-01</P>
                  <P type="APPID">0x0007</P>
                  <P type="VLAN-ID">100</P>
                  <P type="VLAN-PRIORITY">2</P>
                </Address>
              </GSE>
            </GSEControl>
          </LN0>
          <LN lnClass="CSWI" inst="1">
            <Inputs>
              <ExtRef iedName="PROT-L1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Trip" daName="general" serviceType="GOOSE" cbName="GOOSE_TRIP_L1"/>
              <ExtRef iedName="PROT-L2" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Trip" daName="general" serviceType="GOOSE" cbName="GOOSE_TRIP_L2"/>
              <ExtRef iedName="PROT-T1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Trip" daName="general" serviceType="GOOSE" cbName="GOOSE_TRIP_T1"/>
            </Inputs>
          </LN>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

  <IED name="MONITOR-1" type="MonitoringDevice" manufacturer="ManufacturerE">
    <AccessPoint name="GOOSE-A">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
          <LN lnClass="MMXU" inst="1">
            <Inputs>
              <ExtRef iedName="PROT-L1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Signal" daName="stVal" serviceType="GOOSE" cbName="GOOSE_SIGNAL_L1"/>
              <ExtRef iedName="PROT-L2" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Signal" daName="stVal" serviceType="GOOSE" cbName="GOOSE_TRIP_L2"/>
              <ExtRef iedName="BCU-1" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Status" daName="stVal" serviceType="GOOSE" cbName="GOOSE_BCU_STATUS"/>
              <ExtRef iedName="INVALID-IED" apRef="GOOSE-A" ldInst="LD0" prefix="" lnClass="LLN0" lnInst="" doName="Test" daName="stVal" serviceType="GOOSE" cbName="NON_EXISTENT"/>
            </Inputs>
          </LN>
        </LDevice>
      </Server>
    </AccessPoint>

    <AccessPoint name="MMS">
      <Server>
        <LDevice inst="LD0">
          <LN0 lnClass="LLN0" inst=""/>
        </LDevice>
      </Server>
    </AccessPoint>
  </IED>

</SCL>
