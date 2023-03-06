{{/*
Expand the name of the chart.
*/}}
{{- define "kotosiro.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "kotosiro.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "kotosiro.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "kotosiro.labels" -}}
helm.sh/chart: {{ include "kotosiro.chart" . }}
{{ include "kotosiro.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "kotosiro.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kotosiro.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "kotosiro.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "kotosiro.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}


{{/*
Controller.
*/}}
{{- define "kotosiro.controller.name" -}}
{{ include "kotosiro.name" . }}-controller
{{- end }}

{{- define "kotosiro.controller.fullname" -}}
{{ include "kotosiro.fullname" . }}-controller
{{- end -}}

{{- define "kotosiro.controller.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kotosiro.name" . }}-controller
app.kubernetes.io/instance: {{ .Release.Name }}-controller
{{- end }}


{{/*
Database.
*/}}
{{- define "kotosiro.postgres.name" -}}
{{ include "kotosiro.name" . }}-postgres
{{- end }}

{{- define "kotosiro.postgres.fullname" -}}
{{ include "kotosiro.fullname" . }}-postgres
{{- end -}}

{{- define "kotosiro.postgres.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kotosiro.name" . }}-postgres
app.kubernetes.io/instance: {{ .Release.Name }}-postgres
{{- end }}

{{- define "kotosiro.postgres.secretName" -}}
    {{- if .Values.global.postgres.existingSecret -}}
        {{- printf "%s" .Values.global.postgres.existingSecret -}}
    {{- else -}}
        {{- printf "%s" (include "kotosiro.postgres.fullname" .) -}}
    {{- end -}}
{{- end -}}


{{/*
Message Queueing.
*/}}
{{- define "kotosiro.rabbitmq.name" -}}
{{ include "kotosiro.name" . }}-rabbitmq
{{- end }}

{{- define "kotosiro.rabbitmq.fullname" -}}
{{ include "kotosiro.fullname" . }}-rabbitmq
{{- end -}}

{{- define "kotosiro.rabbitmq.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kotosiro.name" . }}-rabbitmq
app.kubernetes.io/instance: {{ .Release.Name }}-rabbitmq
{{- end }}
